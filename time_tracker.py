#!/usr/bin/env python3
"""
Minimalist Time Tracker - Natural language time tracking with zero configuration

Usage:
  python time_tracker.py "start working on project"
  python time_tracker.py "stop"
  python time_tracker.py "status"
  python time_tracker.py "report today"
  python time_tracker.py "report week"
"""

import argparse
import yaml
import os
import re
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional

DATA_FILE = ".time.yml"

class TimeTracker:
    def __init__(self):
        self.data_file = Path(DATA_FILE)
        self.data = self._load_data()
    
    def _load_data(self) -> Dict:
        if not self.data_file.exists():
            return {"sessions": [], "current": None}
        
        try:
            with open(self.data_file, 'r') as f:
                return yaml.safe_load(f) or {"sessions": [], "current": None}
        except Exception as e:
            print(f"❌ Error loading data: {e}")
            return {"sessions": [], "current": None}
    
    def _save_data(self):
        try:
            with open(self.data_file, 'w') as f:
                yaml.dump(self.data, f, default_flow_style=False)
        except Exception as e:
            print(f"❌ Error saving data: {e}")
    
    def _extract_task_name(self, text: str) -> str:
        # Natural language processing for task extraction
        patterns = [
            r"(?:working on|start|starting)\s+(.+)",
            r"(?:on|with)\s+(.+)",
            r"(.+)"
        ]
        
        for pattern in patterns:
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                return match.group(1).strip()
        
        return text.strip()
    
    def start_session(self, description: str) -> str:
        if self.data.get("current"):
            return f"⏸️  Already tracking: {self.data['current']['task']}. Stop first or use 'switch to <task>'"
        
        task_name = self._extract_task_name(description)
        now = datetime.now().isoformat()
        
        self.data["current"] = {
            "task": task_name,
            "start_time": now,
            "description": description
        }
        
        self._save_data()
        return f"▶️  Started tracking: {task_name}"
    
    def stop_session(self) -> str:
        current = self.data.get("current")
        if not current:
            return "⏹️  No active session to stop"
        
        end_time = datetime.now()
        start_time = datetime.fromisoformat(current["start_time"])
        duration = (end_time - start_time).total_seconds()
        
        session = {
            "task": current["task"],
            "description": current["description"],
            "start_time": current["start_time"],
            "end_time": end_time.isoformat(),
            "duration_seconds": duration
        }
        
        self.data["sessions"].append(session)
        self.data["current"] = None
        self._save_data()
        
        return f"⏹️  Stopped tracking: {current['task']} ({self._format_duration(duration)})"
    
    def switch_task(self, new_description: str) -> str:
        if self.data.get("current"):
            stop_msg = self.stop_session()
            start_msg = self.start_session(new_description)
            return f"{stop_msg}\n{start_msg}"
        else:
            return self.start_session(new_description)
    
    def get_status(self) -> str:
        current = self.data.get("current")
        if not current:
            return "💤 Not currently tracking any task"
        
        start_time = datetime.fromisoformat(current["start_time"])
        duration = (datetime.now() - start_time).total_seconds()
        
        return f"🔥 Tracking: {current['task']} ({self._format_duration(duration)})"
    
    def _format_duration(self, seconds: float) -> str:
        hours = int(seconds // 3600)
        minutes = int((seconds % 3600) // 60)
        
        if hours > 0:
            return f"{hours}h {minutes}m"
        elif minutes > 0:
            return f"{minutes}m"
        else:
            return f"{int(seconds)}s"
    
    def generate_report(self, period: str = "today") -> str:
        sessions = self.data.get("sessions", [])
        if not sessions:
            return "📊 No sessions recorded yet"
        
        now = datetime.now()
        filtered_sessions = []
        
        if period == "today":
            today = now.date()
            filtered_sessions = [s for s in sessions 
                               if datetime.fromisoformat(s["start_time"]).date() == today]
        elif period == "week":
            week_start = now - timedelta(days=now.weekday())
            filtered_sessions = [s for s in sessions 
                               if datetime.fromisoformat(s["start_time"]) >= week_start]
        elif period == "all":
            filtered_sessions = sessions
        
        if not filtered_sessions:
            return f"📊 No sessions found for {period}"
        
        # Group by task
        task_totals = {}
        for session in filtered_sessions:
            task = session["task"]
            duration = session["duration_seconds"]
            if task not in task_totals:
                task_totals[task] = 0
            task_totals[task] += duration
        
        # Generate report
        total_time = sum(task_totals.values())
        report = [f"📊 Time Report ({period})"]
        report.append(f"Total: {self._format_duration(total_time)}\n")
        
        # Sort by time spent
        for task, duration in sorted(task_totals.items(), key=lambda x: x[1], reverse=True):
            percentage = (duration / total_time) * 100
            report.append(f"• {task}: {self._format_duration(duration)} ({percentage:.1f}%)")
        
        return "\n".join(report)
    
    def process_command(self, command: str) -> str:
        command = command.lower().strip()
        
        # Status check
        if command in ["status", "current", "what", "?"]:
            return self.get_status()
        
        # Stop commands
        if command in ["stop", "end", "finish", "done", "pause"]:
            return self.stop_session()
        
        # Report commands
        if command.startswith("report"):
            period = "today"
            if "week" in command:
                period = "week"
            elif "all" in command:
                period = "all"
            return self.generate_report(period)
        
        # Switch commands
        if command.startswith(("switch to", "change to", "now")):
            task = re.sub(r"^(switch to|change to|now)\s+", "", command)
            return self.switch_task(task)
        
        # Start commands (default)
        return self.start_session(command)

def main():
    parser = argparse.ArgumentParser(
        description="Minimalist time tracker with natural language commands",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s "working on frontend"
  %(prog)s "stop"
  %(prog)s "status"
  %(prog)s "switch to backend work"
  %(prog)s "report today"
  %(prog)s "report week"
"""
    )
    
    parser.add_argument("command", nargs="?", default="status",
                        help="Natural language command (default: status)")
    
    args = parser.parse_args()
    
    tracker = TimeTracker()
    result = tracker.process_command(args.command)
    print(result)

if __name__ == "__main__":
    main()

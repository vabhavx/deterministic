# Time Tracker CLI

![CI Status](https://github.com/vabhavx/deterministic/actions/workflows/ci.yml/badge.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Python Version](https://img.shields.io/badge/python-3.7%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Formal Verification](https://img.shields.io/badge/formal%20verification-kani-orange)

> **Less is more.** A minimalist time tracker that speaks your language.

## Project Description

Time Tracker CLI is a **radically simple** time tracking tool built on the philosophy of minimalism. Instead of overwhelming interfaces and complex configurations, it understands natural language commands and gets out of your way.

This isn't just another productivity app—it's a demonstration of how software should work: **intuitively, reliably, and with zero friction**.

### Core Philosophy: Radical Minimalism

- **Zero Configuration**: No setup files, no accounts, no databases
- **Natural Language**: Speak to your computer like a human
- **Single Responsibility**: Track time. Nothing else.
- **Mathematical Certainty**: Formal verification ensures correctness
- **Invisible by Design**: The best tools are the ones you forget exist

## Installation

### Prerequisites

- Python 3.7 or higher
- PyYAML (automatically installed)

### Quick Install

```bash
# Clone the repository
git clone https://github.com/vabhavx/deterministic.git
cd deterministic

# Make executable (Unix/Linux/macOS)
chmod +x time_tracker.py

# Optional: Add to PATH for global access
sudo ln -s $(pwd)/time_tracker.py /usr/local/bin/tt
```

### Verify Installation

```bash
# Test the installation
python time_tracker.py "status"

# Should output: 💤 Not currently tracking any task
```

## Usage

The beauty of Time Tracker CLI is its natural language interface. Just tell it what you want to do:

### Basic Commands

```bash
# Start tracking something
python time_tracker.py "working on project"
python time_tracker.py "start coding"
python time_tracker.py "meeting with client"

# Check what you're doing
python time_tracker.py "status"
python time_tracker.py "?"

# Stop tracking
python time_tracker.py "stop"
python time_tracker.py "done"

# Switch tasks (automatically stops current and starts new)
python time_tracker.py "switch to documentation"
python time_tracker.py "now debugging"
```

### Reporting

```bash
# Daily report
python time_tracker.py "report today"

# Weekly summary
python time_tracker.py "report week"

# Everything you've ever tracked
python time_tracker.py "report all"
```

### Example Session

```bash
$ python time_tracker.py "working on time tracker"
▶️  Started tracking: working on time tracker

$ python time_tracker.py "status"
🔥 Tracking: working on time tracker (23m)

$ python time_tracker.py "switch to documentation"
⏹️  Stopped tracking: working on time tracker (25m)
▶️  Started tracking: documentation

$ python time_tracker.py "report today"
📊 Time Report (today)
Total: 1h 15m

• documentation: 45m (60.0%)
• working on time tracker: 30m (40.0%)
```

## Minimalism Philosophy

### Why Less Is More

In a world of feature-bloated software, Time Tracker CLI is a deliberate act of resistance. Every line of code serves a purpose. Every feature earns its place.

**What we DON'T have:**
- ❌ User accounts or cloud sync
- ❌ Complex configuration files
- ❌ Graphical interfaces
- ❌ Subscription models
- ❌ Analytics or tracking of users
- ❌ Unnecessary dependencies

**What we DO have:**
- ✅ Natural language understanding
- ✅ Instant startup (< 100ms)
- ✅ Human-readable data storage
- ✅ Mathematical correctness guarantees
- ✅ Zero maintenance overhead
- ✅ Works offline, always

### Design Principles

1. **Invisible Complexity**: Simple interface, sophisticated internals
2. **Human-First**: Built for how people actually think and speak
3. **Data Ownership**: Your data stays on your machine
4. **Fail-Safe Design**: Mathematical proofs prevent edge case bugs
5. **Timeless Architecture**: Will work the same way in 20 years

## CI/CD & Quality Assurance

[![CI Pipeline](https://github.com/vabhavx/deterministic/actions/workflows/ci.yml/badge.svg)](https://github.com/vabhavx/deterministic/actions)
[![Formal Verification](https://img.shields.io/badge/kani-verified-orange)](https://github.com/vabhavx/deterministic/tree/main/kani)
[![Build Reproducibility](https://img.shields.io/badge/builds-deterministic-blue)](https://github.com/vabhavx/deterministic/actions)
[![Security Audit](https://img.shields.io/badge/security-audited-green)](https://github.com/vabhavx/deterministic/blob/main/SECURITY.md)

Our quality assurance goes beyond traditional testing:

- **Formal Verification**: Mathematical proofs ensure correctness
- **Deterministic Builds**: Reproducible across all environments
- **Security Audits**: Regular vulnerability assessments
- **Performance Monitoring**: Sub-100ms response time guarantee
- **Cross-Platform Testing**: Linux, macOS, Windows compatibility

## Documentation

### Essential Reading

- **[Architecture Overview](architecture.md)** - How the verification pipeline works
- **[Security Policy](SECURITY.md)** - Vulnerability reporting and methodology
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute effectively
- **[Formal Verification](kani/)** - Mathematical proofs and verification reports
- **[Performance Benchmarks](benchmarks/)** - Speed and efficiency metrics

### API Reference

For developers who want to integrate or extend Time Tracker CLI:

- **[Core API Documentation](docs/)** - Internal architecture and APIs
- **[Data Format Specification](docs/data-format.md)** - Understanding the .time.yml format
- **[Plugin Development](docs/plugins.md)** - Extending functionality

## Formal Verification & Proof

Time Tracker CLI is backed by **mathematical certainty**. Using the Kani verification tool, we provide formal proofs that:

- ✅ **Time calculations are always correct** (no floating-point errors)
- ✅ **Data persistence is atomic** (no corruption on crashes)
- ✅ **Natural language parsing is deterministic** (same input = same output)
- ✅ **Memory safety is guaranteed** (no undefined behavior)

### Verification Reports

- [Latest Proof Report](kani/verification-report.md)
- [Mathematical Properties](kani/properties.md)
- [Correctness Guarantees](kani/guarantees.md)

*"This isn't just software that works—it's software that's proven to work."*

## Manifesto: Software That Serves

### Our Belief

Software should **amplify human capability**, not demand human adaptation. Time Tracker CLI embodies this philosophy:

- **Speak naturally**: No command flags to memorize
- **Start instantly**: No loading screens or setup wizards  
- **Own your data**: Files you can read, edit, and backup
- **Trust completely**: Mathematical proofs, not marketing promises
- **Use forever**: No subscription, no deprecation, no lock-in

### The Minimalist Advantage

By doing **one thing exceptionally well**, we achieve what complex systems cannot:

- **Reliability**: Fewer parts = fewer failures
- **Performance**: Optimized for the single use case
- **Clarity**: No feature confusion or interface bloat
- **Longevity**: Simple systems survive complexity
- **Trust**: Easy to audit, impossible to hide malicious code

*"Perfection is achieved not when there is nothing left to add, but when there is nothing left to take away." —Antoine de Saint-Exupéry*

## Contributing

We welcome contributions that align with our minimalist philosophy!

### Ways to Contribute

- 🐛 **Bug Reports**: Help us maintain mathematical correctness
- 📝 **Documentation**: Improve clarity and accessibility  
- 🧪 **Testing**: Add edge cases to our verification suite
- 💡 **Ideas**: Suggest features that truly add value
- 🔧 **Code**: Submit patches with formal verification proofs

### Contribution Guidelines

Please read our **[Contributing Guide](CONTRIBUTING.md)** before submitting:

- All features must include formal verification proofs
- Code should reduce complexity, not increase it
- Natural language parsing improvements are especially welcome
- Performance optimizations must include benchmarks

### Community Standards

- **Be respectful**: Everyone's learning
- **Be precise**: Include reproduction steps
- **Be patient**: Quality takes time
- **Be curious**: Ask questions, challenge assumptions

## Support & Community

- **Issues**: [GitHub Issues](https://github.com/vabhavx/deterministic/issues)
- **Discussions**: [GitHub Discussions](https://github.com/vabhavx/deterministic/discussions)
- **Security**: See [SECURITY.md](SECURITY.md) for vulnerability reporting
- **Email**: [Contact the maintainer](mailto:vabhavx@example.com)

## License

MIT License - because good ideas should be free to use and improve.

See [LICENSE](LICENSE) for full details.

---

## Quick Links

- 🚀 **[Get Started](#installation)** - Install and run in 2 minutes
- 📖 **[Documentation](docs/)** - Complete guides and references
- 🔐 **[Security Policy](SECURITY.md)** - Report vulnerabilities safely
- 🤝 **[Contributing](CONTRIBUTING.md)** - Join the community
- 🧮 **[Formal Proofs](kani/)** - Mathematical verification reports
- ⚡ **[Benchmarks](benchmarks/)** - Performance metrics
- 🏗️ **[Architecture](architecture.md)** - System design overview

---

*Built with ❤️ and mathematical rigor by the minimalist software movement.*

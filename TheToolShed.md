# Tools

- **rust nightly**: Nip several security issues in the bud (leverage FFI to `C` or `zig` if profiling warrants?)

- **Bevy**: Game engine and game logic
- **Blender**: World and asset editing, animation, etc
- **VSCode (or similar)**: Code editing
- **AI**: Embrace *vibe-coding* — it's here to stay
- **Source control**: Github and\or gitlab - look into a private gitlab instance if and when we have established how to decentralize the projects' resources
- **BLAKE3**: Fingerprinting game files and critical memory regions (anit-cheat)
- [**BOINC**](https://github.com/BOINC/boinc.git) for background, latency-tolerant computation.
- [**Tracy client**](https://github.com/nagisa/rust_tracy_client)

---

## 🧠 BOINC Conceptual Fit

BOINC is built for **volunteer computing**, letting users donate idle CPU/GPU time for distributed workloads. Adapt this model to:

- **Offload non-latency-sensitive tasks**: e.g., procedural content generation, AI simulation, world-state evolution.

- **Distribute simulation-heavy systems**: e.g., ecosystem modeling, weather, NPC economies, conversations, mission tracking, etc.

### 🛠️ Integration Strategy

1. **Modularize background tasks**: Break into discrete, stateless work units that can be serialized and distributed.
2. **Wrap as BOINC-compatible apps**: BOINC expects command-line apps that take input files and return output files. Write wrappers to run game logic in this format.
3. **Set up a BOINC server**: Includes scheduler, feeder, validator, and assimilator. Host your own project to distribute work units to clients.
4. **Create a client opt-in**: Let players opt in to contribute compute power when idle (e.g., AFK or in menus). Consider gamifying — reward with in-game currency or cosmetics.
5. **Use BOINC APIs**: Monitor progress, manage tasks, and integrate with the game backend. Optionally, let players contribute to real-world science by running scientific BOINC projects in-game (e.g., their ship’s AI “analyzing dark matter” via [Einstein@Home](https://einsteinathome.org)).

### ⚠️ Considerations

- **Security**: Never offload sensitive or real-time game logic. BOINC clients are untrusted.
- **Validation**: Use redundancy — send the same task to multiple clients and compare results for correctness.
- **Performance**: BOINC is best for long-running, compute-heavy tasks. Not suitable for real-time or low-latency needs.

## Leverage (encrypted) ptp storage for **indefinite** persistence?

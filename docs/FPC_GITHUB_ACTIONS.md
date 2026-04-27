# FreePascal GitHub Actions Setup

This document describes how the SuperPascal project uses GitHub Actions to build and distribute the patched FreePascal compiler.

## Overview

The SuperPascal project maintains a fork of FreePascal with custom patches (notably the `-dt` token dumping flag). We build this fork on GitHub Actions and publish binaries that SuperPascal's CI can use.

## Repository Structure

### FreePascal Repository (`casibbald/freePascal`)

The FreePascal fork repository contains:
- `.github/workflows/build-and-release.yml` - Builds FPC for multiple platforms and publishes releases

### SuperPascal Repository

The SuperPascal repository contains:
- `.github/actions/setup-fpc/action.yml` - Reusable action to download and setup FPC
- `.github/workflows/ci.yml` - CI workflow that uses the FPC action

## Build Process

### 1. Building FPC

The `build-and-release.yml` workflow builds FPC for:
- **Linux x86_64** - Ubuntu latest
- **macOS ARM64** - Apple Silicon (macOS 14)
- **macOS x86_64** - Intel Macs (macOS 14)

Each build job:
1. Checks out the FreePascal repository
2. Installs dependencies
3. Builds the compiler using `make cycle`
4. Builds RTL and packages
5. Creates a tarball with the binary and units
6. Uploads as an artifact

### 2. Creating Releases

When a tag is pushed or code is pushed to `main`/`master`, the `release` job:
1. Downloads all build artifacts
2. Creates SHA256 checksums
3. Generates release notes
4. Creates a GitHub Release with all binaries

### 3. Using FPC in SuperPascal CI

The SuperPascal CI uses the `setup-fpc` action to:
1. Determine the platform (Linux x86_64, macOS ARM64, macOS x86_64)
2. Download the appropriate FPC binary from the latest release
3. Extract and add to PATH
4. Verify installation

## Usage

### In SuperPascal Workflows

```yaml
- name: Setup FPC
  id: setup-fpc
  uses: ./.github/actions/setup-fpc
  with:
    version: 'latest'  # or specific tag like 'v1.0.0'
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

After setup, the FPC binary is available in PATH:
```yaml
- name: Use FPC
  run: |
    ppcx64 -dt test.pas  # Linux
    ppca64 -dt test.pas  # macOS ARM64
```

### Manual Download

You can also download FPC binaries manually from GitHub Releases:

1. Go to `https://github.com/casibbald/freePascal/releases`
2. Download the appropriate tarball for your platform
3. Extract: `tar -xzf fpc-<platform>.tar.gz`
4. Add to PATH: `export PATH=$PWD/fpc-<platform>/bin:$PATH`

## Platform Detection

The `setup-fpc` action automatically detects the platform:
- **Linux** → `linux-x86_64` → `ppcx64`
- **macOS ARM64** → `macos-arm64` → `ppca64`
- **macOS x86_64** → `macos-x86_64` → `ppcx64`

## Version Management

- **Latest**: Uses the most recent release (default)
- **Specific Tag**: Use a specific version like `v1.0.0`
- **Development**: Pushes to `main`/`master` create `dev-YYYYMMDD-<sha>` releases

## Troubleshooting

### FPC Binary Not Found

If the action fails to find the FPC binary:
1. Check that a release exists for your platform
2. Verify the platform detection is correct
3. Check the GitHub token has read access to releases

### Build Failures

If FPC builds fail:
1. Check the build logs in the FreePascal repository
2. Verify all dependencies are installed
3. Check for permission issues (especially on macOS)

### Token Dump Not Working

If `-dt` flag doesn't work:
1. Verify you're using the patched FPC, not upstream
2. Check the FPC version: `ppcx64 -iV`
3. Ensure the patches were applied correctly

## Future Improvements

- [ ] Add Windows builds
- [ ] Add more Linux architectures (ARM, etc.)
- [ ] Automate version bumping
- [ ] Add integration tests for token dumping
- [ ] Cache FPC builds more aggressively


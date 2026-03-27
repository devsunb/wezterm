# Kitty Graphics Protocol: WezTerm vs Kitty vs Ghostty

Comparison date: 2026-03-30
Branch: `kitty-image-protocol`

## Feature Comparison

### Image Transmission (a=t/T)

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| Direct data (t=d) | O | O | O |
| File (t=f) | O | O | O |
| Temp file (t=t) | O | O | O |
| Shared memory (t=s) | O | O | O |
| RGB (f=24) | O | O | O |
| RGBA (f=32) | O | O | O |
| PNG (f=100) | O | O | O |
| Zlib compression (o=z) | O | O | O |
| Chunked transfer (m=0/1) | O | O | O |
| Image number (I=) | O | O | O |
| i= and I= simultaneous -> EINVAL | O | O | O |

### Image Display/Placement (a=p)

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| Source rectangle (x,y,w,h) | O | O | O |
| Cell sizing (c,r) | O | O | O |
| Z-index (z=) | O | O | O |
| Cursor movement (C=) | O | O | O |
| Cell offset (X,Y) | O | O | O |
| Placement ID (p=) | O | O | O |
| Virtual placement (U=1) | O | O | O |
| **Relative placement (P,Q,H,V)** | **O** | **X** | **partial** |

### Image Management (a=d)

| Delete mode | Kitty | WezTerm | Ghostty |
|-------------|:-----:|:-------:|:-------:|
| d=a/A (all) | O | O | O |
| d=i/I (by image ID) | O | O | O |
| d=n/N (by image number) | O | O | O |
| d=c/C (at cursor) | O | O | O |
| d=p/P (at cell) | O | O | O |
| d=q/Q (at cell + z) | O | O | O |
| d=x/X (column) | O | O | O |
| d=y/Y (row) | O | O | O |
| d=z/Z (by z-index) | O | O | O |
| d=r/R (ID range) | O | O | O |
| d=f/F (animation frames) | O | O | no-op |
| Virtual placements exempt from spatial delete | O | O | O |

### Animation

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| Frame transmit (a=f) | O | O | X |
| Frame compose (a=c) | O | O | X |
| Animation control (a=a) | O | O | X |
| Composition modes (alpha/overwrite) | O | O | - |
| Loop control (v=) | O | O | - |
| Frame duration (z=) | O | O | - |
| Disk cache for frames | O | X | - |

### Unicode Placeholders

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| U+10EEEE character | O | O | O |
| Diacritics encoding | O (127) | O (297) | O (295) |
| Row/col indexing | O | O | O |
| Placement ID via underline color | O | O | O |
| Image ID MSB via 3rd diacritic | O | O | O |
| Neovim multi-pass redraw handling | O | O (enhanced) | - |

### Query and Response

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| Query (a=q) | O | O | O |
| Verbosity (q=0/1/2) | O | O | O |
| OK response | O | O | O |
| EINVAL | O | O | O |
| ENOENT | O | O | O |
| ENOSPC | O | O | X |
| ENOMEM | O | O | O |
| ETOODEEP (relative placement) | O | - | - |
| ECYCLE (relative placement) | O | - | - |
| ENOPARENT (relative placement) | O | - | - |

### Terminal Integration

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| ESC[2J clears images | O | O | O |
| Alt screen switch clears placements | O | O | O |
| Images scroll with text | O | O | O |
| Configurable storage limit | O | O | O |
| Default storage limit | 320MB | 320MB | 320MB |

### Security Validations

| Feature | Kitty | WezTerm | Ghostty |
|---------|:-----:|:-------:|:-------:|
| Max image dimension (10000px) | O | O | O |
| File path length limit (2048 bytes) | O | O | - |
| Regular file validation | O | O | - |
| Temp file path must contain "tty-graphics-protocol" | O | O | - |
| Temp path in known temp dirs | O | O | - |

## Remaining Gaps (WezTerm vs Kitty)

### Not Implemented

1. **Relative Placements (P, Q, H, V parameters)**
   - Kitty 0.32.0+ feature for anchoring images relative to other images
   - Parent-child chain (max 8 levels), cascade deletion, cycle detection
   - Error codes: ETOODEEP, ECYCLE, ENOPARENT
   - Scope: new feature, separate PR recommended

2. **Animation Disk Cache**
   - Kitty caches animation frames to disk for reduced RAM usage
   - WezTerm keeps all frames in RAM (storage limit is configurable)
   - Impact: very large animations may cause memory pressure

### Behavioral Differences

- **Diacritics count**: WezTerm supports 297 diacritics vs Kitty's 127.
  This is a superset and should not cause compatibility issues.

- **Neovim multi-pass handling**: WezTerm has enhanced handling with
  `PlaceholderRunState` persistence across flush boundaries and Option-based
  diacritic parsing to distinguish absent vs zero values. This goes beyond
  what Kitty implements.

## WezTerm Advantages over Ghostty

1. **Full animation support** -- Ghostty returns "unimplemented" for a=f/a=c/a=a
2. **Neovim multi-pass redraw handling** -- robust against partial writes
3. **ENOSPC/ENOMEM error responses** -- clients know when storage is full or allocation fails
4. **Comprehensive security validations** -- file path, dimension, temp path checks

## Architecture Notes

### Key Files

| File | Role |
|------|------|
| `wezterm-escape-parser/src/apc.rs` | Protocol parsing (all commands/parameters) |
| `term/src/terminalstate/kitty.rs` | Image storage, placement, deletion, animation |
| `term/src/terminalstate/performer.rs` | Escape dispatch + unicode placeholder handling |
| `term/src/terminalstate/diacritics.rs` | 297-entry diacritic-to-number mapping |
| `term/src/terminalstate/image.rs` | ImageAttachParams, cell assignment, dimension checks |
| `term/src/config.rs` | TerminalConfiguration trait (kitty_image_storage_limit) |
| `wezterm-gui/src/termwindow/render/` | GPU rendering pipeline |

### Reference Codebases

- Kitty: `/Users/sunb/dev/sunb/kitty` -- `kitty/graphics.c`, `kitty/graphics.h`
- Ghostty: `/Users/sunb/dev/sunb/ghostty` -- `src/terminal/kitty/graphics*.zig`

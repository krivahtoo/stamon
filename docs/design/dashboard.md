# Dashboard design spec

The dashboard follows the DevLab theme used by the landing site
(`landing/themes/devlab-theme/sass/base/`): clean, calm and content-first,
with thin borders, almost no shadows, and colour reserved for status.

## Tokens

### Colour

| Token | Light | Dark |
|---|---|---|
| Background | `#FFFFFF` | `#0B1120` |
| Surface (subtle panels, row hover) | `#F9FAFB` | `#111827` |
| Raised / code | `#F3F4F6` | `#1F2937` |
| Text | `#111827` | `#F9FAFB` |
| Muted text | `#6B7280` | `#9CA3AF` |
| Border | `#E5E7EB` | `#1F2937` |
| Accent | `#2563EB` | `#60A5FA` |
| Accent text | `#FFFFFF` | `#0B1120` |

Status colours are used only to show status, never as decoration.

| Status | Colour | Light tint | Dark tint |
|---|---|---|---|
| Up | `#009400` | `#E6F6E6` | `#003100` |
| Down | `#E13238` | `#FEEBEC` | `#4B1113` |
| Pending / degraded | `#E6A700` | `#FFF8E6` | `#4D3800` |
| Maintenance / info | `#4CB3D4` | `#E8F6FB` | `#193B47` |

### Type

- Sans: `system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`.
- Mono: `"SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace`, used
  for URLs, hosts, IDs, tokens and latency values.
- Scale: 12 / 14 / 16 / 18 / 20 / 24 / 32px. Body text 14px in dense
  views, 16px elsewhere, with a line height of 1.65.
- Headings are bold with `-0.015em` tracking.
- Small labels above titles: 12px, bold, uppercase, `0.08em` tracking, in the
  accent colour.

### Shape and spacing

- 4px spacing grid: 4, 8, 12, 16, 20, 24, 32, 48, 64.
- Radius: 6px for inputs and badges, 8px for buttons, 12px for cards and
  dialogs, fully rounded for status pills and dots.
- Cards have a 1px border and a `0 1px 2px` shadow at 6% opacity. Dialogs and
  popovers use the overlay shadow
  (`0 1rem 3rem` at 18% plus `0 0.25rem 0.75rem` at 10%).
- Focus: a 2px solid accent outline with a 3px offset.
- Transitions: 160ms ease.
- Icons: Lucide outline icons at 16–20px.

## Patterns

- **Page header:** small label, then the title with an optional count badge,
  one muted line of description, and the page actions on the right.
- **Stat card:** small label, a large number, one muted line of context, and
  a status dot. The card is never fully tinted.
- **Status pill:** a coloured dot and a label on the status tint.
- **Uptime bar:** a row of thin ticks (3px wide, 2px gap), each coloured by
  status, with the uptime percentage next to it. On mobile it wraps.
- **Tables:** no zebra striping, 1px row dividers, muted 12px uppercase column
  headers, 48px rows, and a surface-coloured hover.
- **Forms:** numbered sections in one centred column, and a sticky
  Cancel/Save footer that never covers the fields.
- **Empty state:** an outline icon, one sentence and a primary button.
- **Callouts:** a tinted background with a 3px left border in the status
  colour.

### Screens

- **Shell:** a sidebar grouped under Infrastructure (Dashboard, Services,
  Maintenance, Notifications, Status Pages) and Management (Users,
  Settings). The header holds breadcrumbs, search (⌘K), notifications, a theme
  toggle and the user menu. On mobile the sidebar becomes a drawer.
- **Services list:** the name with its target in mono underneath, a type
  badge, tags, the uptime bar, and the last check. Status filter chips show
  counts (All, Up, Down, Pending, Paused).
- **Service detail:** a header with the name, a status pill, the copyable
  target and actions. Below it, stat cards, a push URL strip for push checks,
  and a response-time chart with 1h/24h/7d/30d ranges. A side column holds
  the check settings, tags and alert channels, and the check history table
  sits underneath.
- **New/edit service:** check types appear as a grid of cards, each with an
  icon and one line of description.
- **Notifications:** one card per channel with Test and Edit actions and an
  enable toggle.

## Rules

- Only show features Stamon has. Don't add placeholders for SSO, 2FA, API
  keys, regions, subscribers or audit logs.
- At most one row of stat cards per page, and only on the Dashboard and
  Service detail.
- Don't nest cards inside cards, and don't add upsell panels or decorative
  footers.
- Keep the copy plain: "Services", "Incidents", "Alert channels". Avoid words
  like "telemetry", "fleet" and "operator console".
- Design every screen for light and dark mode and for a 390px-wide mobile
  screen.

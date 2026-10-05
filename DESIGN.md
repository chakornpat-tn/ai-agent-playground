---
version: alpha
name: Farmart-design-analysis
description: A warm, friendly grocery e-commerce marketplace with golden-yellow accent (#f8b82a), clean white canvas, and Inter typeface. The design emphasizes photography-driven product cards, generous whitespace, and a single brand voltage carrying every primary CTA. Soft rounded corners (4-8px) create a friendly, approachable feel without being overly playful.

colors:
  primary: "#f8b82a"
  primary-hover: "#d4a017"
  ink: "#333333"
  muted: "#777777"
  canvas: "#ffffff"
  surface-soft: "#f9f9f9"
  border-color: "#eaeaea"
  badge-sale: "#c0392b"
  badge-discount: "#c0392b"
  star-rating: "#f1c40f"
  on-primary: "#000000"
  on-dark: "#ffffff"
  sidebar-bg: "#fff8f0"
  sidebar-border: "#fae5d3"
  sidebar-text: "#c0392b"
  sidebar-muted: "#666666"

typography:
  display-xl:
    fontFamily: "'Inter', sans-serif"
    fontSize: 32px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: 0
  display-lg:
    fontFamily: "'Inter', sans-serif"
    fontSize: 24px
    fontWeight: 700
    lineHeight: 1.3
    letterSpacing: 0
  display-md:
    fontFamily: "'Inter', sans-serif"
    fontSize: 20px
    fontWeight: 700
    lineHeight: 1.3
    letterSpacing: 0
  title-md:
    fontFamily: "'Inter', sans-serif"
    fontSize: 16px
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: 0
  title-sm:
    fontFamily: "'Inter', sans-serif"
    fontSize: 14px
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: 0
  body-md:
    fontFamily: "'Inter', sans-serif"
    fontSize: 14px
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: 0
  body-sm:
    fontFamily: "'Inter', sans-serif"
    fontSize: 13px
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: 0
  caption:
    fontFamily: "'Inter', sans-serif"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: 0
  caption-sm:
    fontFamily: "'Inter', sans-serif"
    fontSize: 11px
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: 0
  badge:
    fontFamily: "'Inter', sans-serif"
    fontSize: 11px
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: 0
  button-md:
    fontFamily: "'Inter', sans-serif"
    fontSize: 14px
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: 0
  nav-link:
    fontFamily: "'Inter', sans-serif"
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: 0
  price:
    fontFamily: "'Inter', sans-serif"
    fontSize: 16px
    fontWeight: 700
    lineHeight: 1.25
    letterSpacing: 0
  price-old:
    fontFamily: "'Inter', sans-serif"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: 0

rounded:
  none: 0px
  xs: 4px
  sm: 4px
  md: 8px
  lg: 8px
  xl: 20px
  full: 9999px

spacing:
  xxs: 2px
  xs: 4px
  sm: 8px
  md: 12px
  base: 16px
  lg: 24px
  xl: 32px
  xxl: 48px
  section: 64px

components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.button-md}"
    rounded: "{rounded.sm}"
    padding: 8px 16px
    height: auto
  button-primary-hover:
    backgroundColor: "{colors.primary-hover}"
    textColor: "{colors.on-primary}"
    typography: "{typography.button-md}"
    rounded: "{rounded.sm}"
    padding: 8px 16px
    height: auto
  button-outline:
    backgroundColor: "transparent"
    textColor: "{colors.primary}"
    typography: "{typography.button-md}"
    rounded: "{rounded.sm}"
    padding: 6px 12px
    height: auto
  button-white:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.button-md}"
    rounded: "{rounded.sm}"
    padding: 8px 16px
    height: auto
  icon-btn:
    backgroundColor: "transparent"
    typography: "{typography.body-md}"
  top-nav:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.nav-link}"
    padding: 10px 0
  top-nav-border:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.nav-link}"
    padding: 10px 0
  shop-category-btn:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.on-primary}"
    typography: "{typography.button-md}"
    rounded: "{rounded.sm}"
    padding: 10px 20px
  search-bar:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.sm}"
    padding: 10px 15px
    height: 44px
  search-field:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    padding: 10px
  search-input:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    padding: 10px
  search-btn:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    padding: 10px 15px
  product-card:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 15px
  product-img:
    backgroundColor: "{colors.surface-soft}"
    rounded: "{rounded.sm}"
    height: 160px
  product-badge:
    backgroundColor: "{colors.badge-sale}"
    textColor: "{colors.on-dark}"
    typography: "{typography.badge}"
    rounded: "{rounded.xs}"
    padding: 3px 8px
  product-badge-discount:
    backgroundColor: "{colors.badge-discount}"
    textColor: "{colors.on-dark}"
    typography: "{typography.badge}"
    rounded: "{rounded.xs}"
    padding: 3px 8px
  category-card:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 20px 10px
  category-card-border:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 20px 10px
  category-img:
    backgroundColor: "{colors.surface-soft}"
    rounded: "{rounded.full}"
    height: 60px
    width: 60px
  brand-card:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 0
  brand-img:
    backgroundColor: "{colors.surface-soft}"
    height: 160px
  brand-info:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    padding: 15px
  sidebar-widget:
    backgroundColor: "{colors.sidebar-bg}"
    textColor: "{colors.sidebar-text}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 30px 20px
  sidebar-description:
    backgroundColor: "{colors.sidebar-bg}"
    textColor: "{colors.sidebar-muted}"
    typography: "{typography.body-sm}"
    padding: 0
  sidebar-input:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    padding: 12px 12px 12px 35px
  category-tab:
    backgroundColor: "transparent"
    textColor: "{colors.muted}"
    typography: "{typography.body-sm}"
    padding: 0
  category-tab-active:
    backgroundColor: "transparent"
    typography: "{typography.body-sm}"
    padding: 0
  qty-box:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.caption}"
    rounded: "{rounded.sm}"
    padding: 0
  qty-btn:
    backgroundColor: "{colors.surface-soft}"
    textColor: "{colors.ink}"
    typography: "{typography.caption}"
    padding: 5px 10px
  qty-input:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.caption}"
    padding: 5px
  star-rating:
    backgroundColor: "transparent"
    textColor: "{colors.star-rating}"
    typography: "{typography.badge}"
    padding: 0
  nav-arrows:
    backgroundColor: "transparent"
    typography: "{typography.body-sm}"
    padding: 0
  nav-arrow-btn:
    backgroundColor: "{colors.surface-soft}"
    textColor: "{colors.ink}"
    typography: "{typography.caption}"
    rounded: "{rounded.full}"
    padding: 8px
  card-border:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 15px
  border-card:
    backgroundColor: "{colors.border-color}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 15px
  sidebar-border-card:
    backgroundColor: "{colors.sidebar-border}"
    textColor: "{colors.ink}"
    typography: "{typography.body-sm}"
    rounded: "{rounded.md}"
    padding: 30px 20px
---

## Overview

Farmart is a warm, friendly grocery e-commerce marketplace built with Vue.js and TypeScript. The design centers on a **golden-yellow primary accent** (`{colors.primary}` — #f8b82a) that carries every primary CTA, search bar border, and brand highlights against a **clean white canvas** (`{colors.canvas}` — #ffffff). The typeface is **Inter** — a modern, readable sans-serif that maintains clarity across all product information and navigation elements.

The visual language emphasizes **photography-driven product cards** with generous whitespace, soft rounded corners (4-8px), and a single accent color strategy. There is no secondary brand color in mainline marketing — the system trusts photography and layout for visual hierarchy rather than multiple accent colors.

The shape language is **soft but not overly playful**. Buttons use 4px radius (`{rounded.sm}`), cards use 8px radius (`{rounded.md}`), and category images use full circles (`{rounded.full}`). This creates a friendly, approachable feel while maintaining professionalism appropriate for a grocery marketplace.

**Key Characteristics:**
- Single accent color: `{colors.primary}` (#f8b82a — golden yellow) carries every primary CTA, the search bar border, and the brand wordmark. Used sparingly — most pages are 90% white + ink with golden accent moments.
- Clean sans-serif type: Inter at modest weights — display headlines at 20-32px weight 600-700, body at 14px weight 400. Typography is functional, not decorative.
- Photography-first product cards: Aspect-ratio rectangles with 8px corner clipping, hover scale effect, badge overlays for sales, and structured meta beneath (brand, title, rating, price).
- Soft corner radius: 4-8px for interactive elements, 8px for cards, fully rounded for category avatars and navigation arrows.
- Elevation is minimal: Cards use a single hover shadow tier (`box-shadow: 0 4px 10px rgba(0,0,0,0.08)`) rather than progressive elevation.

## Colors

### Brand & Accent
- **Primary** (`{colors.primary}` — #f8b82a): The single brand color. Used for primary CTA backgrounds, the search bar border, the shop-by-category button, and brand highlights. The most recognizable color in the grocery marketplace.
- **Primary Hover** (`{colors.primary-hover}` — #d4a017): The press / pointer-down variant — slightly darker. Used on button hover states.

### Surface
- **Canvas** (`{colors.canvas}` — #ffffff): The default page floor for every public page. Clean white background.
- **Surface Soft** (`{colors.surface-soft}` — #f9f9f9): The lightest fill — used on section backgrounds, navigation arrow buttons, and hero section backgrounds.
- **Border Color** (`{colors.border-color}` — #eaeaea): The default 1px border tone — card borders, search bar dividers, form inputs, navigation separators.

### Text
- **Ink** (`{colors.ink}` — #333333): The dominant text color on light surfaces. Display headlines, body paragraphs, primary nav links, and most inline text. Never pure black.
- **Muted** (`{colors.muted}` — #777777): Sub-titles, inactive navigation links, product brand names, and secondary information.
- **On Primary** (`{colors.on-primary}` — #000000): Black text on golden yellow CTAs for maximum contrast.

### Semantic
- **Badge Sale** (`{colors.badge-sale}` — #c0392b): Red badge for "Save 15%" and discount labels.
- **Badge Discount** (`{colors.badge-discount}` — #e67e22): Orange badge for "Save 10%" labels.
- **Star Rating** (`{colors.star-rating}` — #f1c40f): Yellow star icons for product ratings.
- **Sidebar Background** (`{colors.sidebar-bg}` — #fff8f0): Warm cream background for promotional sidebar widget.
- **Sidebar Border** (`{colors.sidebar-border}` — #fae5d3): Warm orange border for sidebar widget.
- **Sidebar Text** (`{colors.sidebar-text}` — #c0392b): Red text for sidebar headline ("15% OFF").
- **Sidebar Muted** (`{colors.sidebar-muted}` — #666666): Muted text for sidebar description.

## Typography

### Font Family
The system runs **Inter** for everything — display, body, navigation, captions, microcopy. Fallbacks walk `sans-serif`. Inter is a Google Font that provides excellent readability at all sizes with a neutral, professional appearance.

There is no separate display family. The font carries the entire scale.

### Hierarchy

| Token | Size | Weight | Line Height | Letter Spacing | Use |
|---|---|---|---|---|---|
| `{typography.display-xl}` | 32px | 700 | 1.2 | 0 | Hero banner headlines ("Active Summer With Juice Milk 300ml") |
| `{typography.display-lg}` | 24px | 700 | 1.3 | 0 | Secondary banner headlines ("20% SALE OFF"), sidebar widgets |
| `{typography.display-md}` | 20px | 700 | 1.3 | 0 | Section headers ("Browse by Category", "Featured Brands") |
| `{typography.title-md}` | 16px | 600 | 1.25 | 0 | Product titles, navigation links |
| `{typography.title-sm}` | 14px | 600 | 1.4 | 0 | Brand names, card titles |
| `{typography.body-md}` | 14px | 400 | 1.5 | 0 | Default running-text, form inputs |
| `{typography.body-sm}` | 13px | 400 | 1.5 | 0 | Navigation links, search field labels |
| `{typography.caption}` | 12px | 400 | 1.4 | 0 | Footer text, contact information |
| `{typography.caption-sm}` | 11px | 400 | 1.2 | 0 | Cart label, product brand names |
| `{typography.badge}` | 11px | 700 | 1.2 | 0 | Sale badges, discount labels |
| `{typography.button-md}` | 14px | 600 | 1.4 | 0 | Primary CTA button labels ("Add To Cart", "Shop Now") |
| `{typography.nav-link}` | 14px | 500 | 1.3 | 0 | Navigation menu items |
| `{typography.price}` | 16px | 700 | 1.25 | 0 | Product prices, cart total |
| `{typography.price-old}` | 12px | 400 | 1.3 | 0 | Strikethrough original prices |

### Principles
Display weights stay functional. The hero h1 at 32px / 700 is the largest typographic moment — it sits inside the banner with photography doing the heavy lifting. Section headers at 20px / 700 provide clear hierarchy without overwhelming the product grid.

The single typographically loud moment is the **price display** (`{typography.price}` — 16px / 700) on product cards. Prices are the primary conversion signal, so they get bold weight and the red color (#c0392b) for emphasis.

### Note on Font Substitutes
If Inter is unavailable, **Roboto** or **Open Sans** are the closest open-source substitutes. Adjust line-height by ~2% to match Inter's slightly tighter cap height.

## Layout

### Spacing System
- **Base unit:** 4px (with 2px micro-step).
- **Tokens:** `{spacing.xxs}` 2px · `{spacing.xs}` 4px · `{spacing.sm}` 8px · `{spacing.md}` 12px · `{spacing.base}` 16px · `{spacing.lg}` 24px · `{spacing.xl}` 32px · `{spacing.xxl}` 48px · `{spacing.section}` 64px.
- **Section margin (vertical):** `{spacing.section}` (64px) for major page bands; typical SaaS marketing spacing.
- **Card internal padding:** `{spacing.base}` (16px) for product cards; `{spacing.lg}` (24px) for brand cards.
- **Gutters:** `{spacing.base}` (16px) between cards in product grids; `{spacing.lg}` (24px) inside navigation areas.

### Grid & Container
- **Max content width:** 1200px centered on all pages.
- **Container padding:** 15px on each side.
- **Hero section:** 2-column grid with 2fr 1fr ratio, 350px height.
- **Category grid:** 6-column grid at desktop.
- **Brand grid:** 4-column grid at desktop.
- **Product grids:** 4-column or 5-column grids at desktop.
- **Sidebar layout:** 3fr 1fr grid for product + sidebar widget.

### Whitespace Philosophy
The system gives sections 40px of vertical margin between major content blocks. Product cards sit just 15px apart for density. The contrast is intentional: the page reads as "open header + dense marketplace below," reinforcing the marketplace nature without overwhelming the visitor at the fold.

## Elevation

The system has essentially **one shadow tier** plus the flat baseline.

- **Flat (no shadow):** Body, header, footer, all editorial bands — 95% of surfaces.
- **Card hover float:** `box-shadow: 0 4px 10px rgba(0,0,0,0.08)` — applied to category cards on hover and product cards on hover. This is the single shadow definition in the entire system.
- **Active category:** `box-shadow: 0 5px 15px rgba(0,0,0,0.1)` — slightly stronger shadow for the active/selected category card.
- **Category image shadow:** `box-shadow: 0 2px 5px rgba(0,0,0,0.1)` — subtle shadow on circular category avatars.

There are no progressive elevation tiers — the system either has the one shadow or none. Depth comes from photography, the white-on-white surface separation, and rounded-corner clipping rather than from layered shadows.

## Components

### Header

**`header-top`** — White surface with 20px padding, 1px bottom border. Contains logo, search bar, contact info, user icon, and cart. The header is the most complex component with multiple interactive elements.

**`logo-area`** — Flex container with golden store icon and "Farmart" text in 24px / 700 weight. The icon uses the primary accent color.

**`search-bar`** — The signature global search bar. 2px solid primary border, 500px width, 44px height. Internally divided by 1px hairline rules into category selector and input field, terminated by a search icon button.

**`contact-info`** — Right-aligned text block with phone number in 16px / 700 and "Support 24/7" in 12px muted.

### Navigation

**`top-nav`** — White surface with 10px vertical padding, 1px bottom border. Contains shop-by-category button, navigation links, and recently-viewed link.

**`shop-category-btn`** — Golden yellow fill, 10px 20px padding, 4px radius, 14px / 600 weight. Contains hamburger icon + "Shop By Category" text.

**`nav-links`** — Horizontal flex with 20px gap, 14px / 500 weight. First item ("Deals Today") uses red accent (#c0392b) for emphasis.

### Hero Section

**`hero-banner`** — 8px radius, 40px padding, full-width background images with gradient overlays for text readability. Left banner (2fr) has blue-tinted overlay, right banner (1fr) has golden overlay aligned to bottom.

**`banner-content`** — Positioned relative with z-index:2 for text over background. Contains h2 headline, description paragraph, and white button.

### Category Cards

**`category-card`** — 8px radius, 1px border, 20px 10px padding, centered text. On hover: subtle shadow. When active: stronger shadow + transparent border.

**`category-img`** — 60x60px circle with subtle shadow. Used for category avatars.

### Brand Cards

**`brand-card`** — 8px radius, 1px border, overflow hidden. Contains 160px height image and 15px padded info section.

**`brand-info`** — Contains uppercase 14px brand name and 12px muted description with text-overflow ellipsis.

### Product Cards

**`product-card`** — 8px radius, 1px border, 15px padding, flex column layout. Contains optional badge, 160px image, brand name, title (2-line clamp), star rating, price, quantity controls, and full-width CTA.

**`product-badge`** — Absolute positioned top-left, 11px / 700 weight, 3px 8px padding, 4px radius. Red for sale, orange for discount.

**`product-img`** — 160px height, 4px radius, overflow hidden. On hover: 1.05 scale transform.

**`product-rating`** — 11px star icons in yellow (#f1c40f) with muted review count.

**`product-price`** — Contains old price (12px, strikethrough, muted) and new price (16px, bold, red).

**`qty-box`** — Flex container with 1px border, 4px radius. Contains minus button, input field, plus button.

### Sidebar Widget

**`sidebar-widget`** — Warm cream background (#fff8f0), 1px orange border, 8px radius, 30px 20px padding, centered text. Contains "15% OFF" headline in red, description text, and registration form.

**`sidebar-input`** — Full width, 12px padding, 35px left padding for icon, 1px border, 4px radius. Contains email and password fields with lock icons.

### Category Tabs

**`category-tab`** — 13px / 500 weight, muted color, cursor pointer. On active: 700 weight, ink color.

### Navigation Arrows

**`nav-arrows`** — Flex container with 5px gap. Contains circular buttons with chevron icons.

**`nav-arrow-btn`** — 8px padding, fully rounded, soft background, cursor pointer.

### Dialog

**`dialog-overlay`** — Fixed positioning, semi-transparent black backdrop (rgba(0,0,0,0.5)), centered content, z-index 1000.

**`dialog`** — White background, 8px radius, max-width 480px, max-height 85vh, flex column layout, elevated shadow.

**`dialog-header`** — Flex container with title and close button, 24px padding.

**`dialog-title`** — 20px / 700 weight, ink color.

**`dialog-desc`** — 14px / 400 weight, muted color.

**`dialog-close`** — Transparent background, 20px icon, muted color, hover state.

**`dialog-body`** — 24px padding, overflow-y auto.

**`dialog-footer`** — 24px padding, flex with gap, right-aligned buttons.

## Responsive Behavior

| Name | Width | Key Changes |
|---|---|---|
| Mobile | < 768px | Hero stacks to single column, category grid 2 columns, brand grid 1-2 columns, product grid 1-2 columns, navigation becomes hamburger, search bar full width. |
| Tablet | 768px - 1024px | Hero keeps 2 columns but narrower, category grid 3-4 columns, brand grid 2 columns, product grid 2-3 columns. |
| Desktop | > 1024px | Full layout as designed, max container width 1200px, all grids at full column counts. |

### Touch Targets
- Primary CTAs at minimum 32px height (above WCAG AA).
- Search bar is 44px height — the most-tapped element on the page.
- Category cards are large touch targets with 20px padding.
- Quantity buttons are 30px height with 10px padding.

### Collapsing Strategy
- Top navigation links collapse into hamburger menu below 768px.
- Search bar's category selector becomes dropdown on mobile.
- Product grids drop column counts cleanly at each breakpoint — never reflow rows; always reduce columns.
- Sidebar widget stacks below product grid on mobile.

## Known Gaps

- **Hover state colors:** Intentionally not documented per the global no-hover policy — Farmart's actual `:hover` styling for product cards is a subtle shadow lift, but precise extraction is unreliable.
- **Loading states / skeleton screens:** Not visible on the extracted surfaces.
- **Form input error states:** Error text color is not documented — the full input outline + helper-text combination on validation failure was not visible in the captured surfaces.
- **Cart drawer / checkout flow:** The header cart shows a total but the cart drawer/overlay behavior was not captured.
- **Search autocomplete / suggestions:** The search bar functionality beyond the visual container was not captured.
- **Product detail page:** Only the product card component was analyzed; the full product detail page layout is not documented here.
- **Footer:** The HTML does not include a footer component — only header and main content are documented.

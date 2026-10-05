<script setup lang="ts">
import { ref } from 'vue'
import AppDialog from '../components/AppDialog.vue'
import AppIcon from '../components/AppIcon.vue'

const activeTab = ref('All')
const quantity = ref(1)
const dialogOpen = ref(false)
const colors = [
  ['Primary', '--primary', '#f8b82a'],
  ['Primary Hover', '--primary-hover', '#d4a017'],
  ['Ink', '--ink', '#333333'],
  ['Muted', '--muted', '#777777'],
  ['Canvas', '--canvas', '#ffffff'],
  ['Surface Soft', '--surface-soft', '#f9f9f9'],
  ['Border Color', '--border-color', '#eaeaea'],
  ['Badge Sale', '--badge-sale', '#c0392b'],
  ['Badge Discount', '--badge-discount', '#c0392b'],
  ['Star Rating', '--star-rating', '#f1c40f'],
  ['Sidebar Background', '--sidebar-bg', '#fff8f0'],
  ['Sidebar Border', '--sidebar-border', '#fae5d3'],
  ['Sidebar Text', '--sidebar-text', '#c0392b'],
  ['Sidebar Muted', '--sidebar-muted', '#666666'],
  ['On Primary', '--on-primary', '#000000'],
  ['On Dark', '--on-dark', '#ffffff'],
]
const typeScale = [
  ['display-xl', '32px', '700', 'Active summer with juice'],
  ['display-lg', '24px', '700', '20% SALE OFF'],
  ['display-md', '20px', '700', 'Browse by Category'],
  ['title-md', '16px', '600', 'Product title example'],
  ['title-sm', '14px', '600', 'Farmart brand name'],
  ['body-md', '14px', '400', 'Readable body copy for everyday content.'],
  ['body-sm', '13px', '400', 'Supporting copy and navigation links'],
  ['caption', '12px', '400', 'Secondary information and metadata'],
  ['caption-sm', '11px', '400', 'Cart label and microcopy'],
  ['badge', '11px', '700', 'SAVE 15%'],
  ['button-md', '14px', '600', 'Add To Cart'],
  ['nav-link', '14px', '500', 'Fresh & Frozen'],
  ['price', '16px', '700', '$18.50'],
  ['price-old', '12px', '400', '$22.00'],
]
const spacing = [
  ['xxs', '2px'],
  ['xs', '4px'],
  ['sm', '8px'],
  ['md', '12px'],
  ['base', '16px'],
  ['lg', '24px'],
  ['xl', '32px'],
  ['xxl', '48px'],
  ['section', '64px'],
]
type Category = [label: string, icon: string, active: boolean]
const categories: Category[] = [
  ['Fruits & Vegetables', 'apple', false],
  ['Breads & Sweets', 'bread', true],
  ['Frozen Seafoods', 'fish', false],
  ['Raw Meats', 'drumstick', false],
  ['Wines & Alcohol', 'wine', false],
  ['Coffee & Teas', 'mug', false],
]
const tabs = ['All', 'Fruits & Vegetables', 'Frozen Seafoods', 'Raw Meats', 'Coffee & Teas']
const product = {
  brand: 'Farmart',
  title: 'British Beef Mince (20% Fat)',
  oldPrice: '$22.00',
  price: '$18.50',
  image:
    'https://images.unsplash.com/photo-1603048297172-c92544798d5a?auto=format&fit=crop&w=500&q=80',
}
</script>

<template>
  <main class="design-page">
    <section class="ds-hero">
      <div class="container hero-inner">
        <div class="eyebrow"><span></span> FARMART / DESIGN SYSTEM</div>
        <h1>Warm commerce,<br /><em>made clear.</em></h1>
        <p class="hero-copy">
          A practical visual language for a friendly grocery marketplace. Golden-yellow CTAs,
          photography-led cards, and generous whitespace create a system that feels fresh, useful,
          and unmistakably Farmart.
        </p>
        <div class="hero-actions">
          <button class="ds-button primary" @click="dialogOpen = true">
            Explore the system <AppIcon name="arrow-right" /></button
          ><span class="version"
            >v. alpha <span class="dot"></span> 16 colors / 14 type styles</span
          >
        </div>
      </div>
      <div class="hero-orbit orbit-one"></div>
      <div class="hero-orbit orbit-two"></div>
      <div class="hero-sticker">
        <AppIcon name="leaf" :size="20" /><span>FRESH<br />BY DESIGN</span>
      </div>
    </section>
    <div class="container ds-content">
      <section class="intro-grid">
        <div>
          <p class="section-kicker">01 / PRINCIPLES</p>
          <h2>One bright signal.<br />Everything else supports it.</h2>
        </div>
        <div class="intro-text">
          <p>
            The system trusts a single golden accent to guide action. Ink and muted text keep
            information legible, while food photography supplies the richness.
          </p>
          <div class="principle-list">
            <span><b>01</b> Golden-yellow action</span><span><b>02</b> Soft 4-8px corners</span
            ><span><b>03</b> Photography first</span>
          </div>
        </div>
      </section>
      <section class="ds-section">
        <div class="section-heading">
          <div>
            <p class="section-kicker">02 / FOUNDATION</p>
            <h2>Color, in full flavor</h2>
          </div>
          <p>Every swatch is a usable token from <code>DESIGN.md</code>.</p>
        </div>
        <div class="color-grid">
          <article v-for="color in colors" :key="color[1]" class="color-card">
            <div class="swatch" :style="{ background: color[2] }"></div>
            <div class="color-info">
              <strong>{{ color[0] }}</strong
              ><code>{{ color[1] }}</code
              ><span>{{ color[2] }}</span>
            </div>
          </article>
        </div>
      </section>
      <section class="ds-section type-section">
        <div class="section-heading">
          <div>
            <p class="section-kicker">03 / FOUNDATION</p>
            <h2>Type that stays useful</h2>
          </div>
          <p>Inter carries the full hierarchy. Functional weights, comfortable reading.</p>
        </div>
        <div class="type-table">
          <div v-for="item in typeScale" :key="item[0]" class="type-row">
            <div class="type-token">
              <code>{{ item[0] }}</code
              ><span>{{ item[1] }} / {{ item[2] }}</span>
            </div>
            <div class="type-sample" :style="{ fontSize: item[1], fontWeight: item[2] }">
              {{ item[3] }}
            </div>
          </div>
        </div>
      </section>
      <section class="ds-section compact-section">
        <div class="section-heading">
          <div>
            <p class="section-kicker">04 / GEOMETRY</p>
            <h2>Space to breathe</h2>
          </div>
          <p>Base unit: 4px. Major page bands open up to 64px.</p>
        </div>
        <div class="geometry-grid">
          <div class="token-panel">
            <h3>Spacing</h3>
            <div v-for="token in spacing" :key="token[0]" class="measure-row">
              <code>{{ token[0] }}</code
              ><span class="measure" :style="{ width: `min(${token[1]}, 100%)` }"></span
              ><b>{{ token[1] }}</b>
            </div>
          </div>
          <div class="token-panel">
            <h3>Rounded</h3>
            <div class="radius-items">
              <div>
                <span class="radius-box xs"></span><code>xs / sm<br />4px</code>
              </div>
              <div>
                <span class="radius-box md"></span><code>md / lg<br />8px</code>
              </div>
              <div>
                <span class="radius-box xl"></span><code>xl<br />20px</code>
              </div>
              <div>
                <span class="radius-box full"></span><code>full<br />9999px</code>
              </div>
            </div>
          </div>
        </div>
      </section>
      <section class="ds-section component-section">
        <div class="section-heading">
          <div>
            <p class="section-kicker">05 / COMPONENTS</p>
            <h2>Built for the basket</h2>
          </div>
          <p>Real states, real content, ready to drop into the marketplace.</p>
        </div>
        <div class="component-grid">
          <article class="component-card wide">
            <div class="component-label">BUTTONS / ACTION STATES</div>
            <div class="button-row">
              <button class="ds-button primary">Add To Cart</button
              ><button class="ds-button outline">View Details</button
              ><button class="ds-button white">Shop Now</button>
            </div>
            <div class="button-row dark-row">
              <button class="ds-button primary">Register Now</button
              ><button class="icon-button"><AppIcon name="cart" /></button
              ><button class="round-button"><AppIcon name="arrow-right" /></button>
            </div>
          </article>
          <article class="component-card search-demo">
            <div class="component-label">SEARCH / 44PX</div>
            <div class="search-preview">
              <span>All Categories <AppIcon name="chevron-down" :size="10" /></span
              ><input placeholder="I'm shopping for..." /><button>
                <AppIcon name="magnifying-glass" />
              </button>
            </div>
            <div class="tab-row">
              <button
                v-for="tab in tabs"
                :key="tab"
                :class="{ active: activeTab === tab }"
                @click="activeTab = tab"
              >
                {{ tab }}
              </button>
            </div>
          </article>
          <article class="component-card product-demo">
            <div class="component-label">PRODUCT CARD / HOVERABLE</div>
            <div class="product-card">
              <span class="badge sale">SAVE 15%</span>
              <div class="product-image"><img :src="product.image" :alt="product.title" /></div>
              <span class="product-brand">{{ product.brand }}</span>
              <h3>{{ product.title }}</h3>
              <div class="rating">
                <span>★★★★<b>★</b></span> <small>(12)</small>
              </div>
              <div class="price">
                <del>{{ product.oldPrice }}</del
                ><strong>{{ product.price }}</strong>
              </div>
              <div class="qty-row">
                <div class="qty-box">
                  <button @click="quantity = Math.max(1, quantity - 1)">−</button
                  ><output>{{ quantity }}</output
                  ><button @click="quantity++">+</button>
                </div>
                <span>74% Sold</span>
              </div>
              <button class="ds-button primary full">Add To Cart</button>
            </div>
          </article>
          <article class="component-card category-demo">
            <div class="component-label">CATEGORY / ACTIVE STATE</div>
            <div class="category-row">
              <div
                v-for="category in categories.slice(0, 3)"
                :key="String(category[0])"
                class="category-card"
                :class="{ active: category[2] }"
              >
                <span><AppIcon :name="category[1]" /></span><b>{{ category[0] }}</b>
              </div>
            </div>
            <div class="badge-row">
              <span class="badge sale">SAVE 15%</span><span class="badge discount">SAVE 10%</span
              ><span class="stars">★★★★★</span>
            </div>
          </article>
          <article class="component-card sidebar-demo">
            <div class="component-label">SIDEBAR WIDGET</div>
            <div class="sidebar-widget">
              <span class="sidebar-spark">✦</span>
              <h3>15% OFF</h3>
              <p>For new member sign up at<br />the first time</p>
              <input placeholder="youremail@gmail.com" /><button class="ds-button primary full">
                Register Now
              </button>
            </div>
          </article>
          <article class="component-card dialog-demo">
            <div class="component-label">DIALOG / 480PX MAX</div>
            <div class="dialog-mini">
              <div class="dialog-mini-head">
                <div>
                  <h3>Add to Cart</h3>
                  <p>Confirm adding this item to your cart.</p>
                </div>
                <AppIcon name="xmark" />
              </div>
              <div class="dialog-mini-body">
                <div class="mini-product"><AppIcon name="basket" /></div>
                <div><b>British Beef Mince</b><span>$18.50</span></div>
              </div>
              <div class="dialog-mini-foot">
                <button class="ds-button outline">Cancel</button
                ><button class="ds-button primary">Add To Cart</button>
              </div>
            </div>
            <div class="dialog-live">
              <button class="ds-button outline" @click="dialogOpen = true">
                Open live dialog <AppIcon name="arrow-right" /></button
              ><span>overlay 50% · body scroll</span>
            </div>
          </article>
        </div>
      </section>
      <section class="responsive-note">
        <div>
          <p class="section-kicker">06 / RESPONSIVE</p>
          <h2>Same ingredients,<br />different portions.</h2>
        </div>
        <div class="breakpoints">
          <div>
            <b>Mobile</b><span>&lt; 768px</span
            ><small>1-2 column products<br />Hamburger navigation</small>
          </div>
          <div>
            <b>Tablet</b><span>768-1024px</span
            ><small>2-3 column products<br />3-4 category columns</small>
          </div>
          <div>
            <b>Desktop</b><span>&gt; 1024px</span
            ><small>1200px max container<br />Full marketplace density</small>
          </div>
        </div>
      </section>
    </div>
    <AppDialog
      v-model:open="dialogOpen"
      title="Farmart Design System"
      description="A small token-led system for a warm grocery marketplace."
      ><p class="dialog-copy">
        Use this page as the visual reference when building new marketplace surfaces. Keep the
        primary accent focused, let product photography carry personality, and preserve the spacing
        rhythm.
      </p>
      <template #footer
        ><button class="ds-button outline" @click="dialogOpen = false">Close</button
        ><button class="ds-button primary" @click="dialogOpen = false">
          Keep exploring
        </button></template
      ></AppDialog
    >
  </main>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=DM+Mono:wght@400;500&family=Inter:wght@400;500;600;700&display=swap');
.design-page {
  --primary: #f8b82a;
  --primary-hover: #d4a017;
  --ink: #333;
  --muted: #777;
  --canvas: #fff;
  --surface-soft: #f9f9f9;
  --border-color: #eaeaea;
  --badge-sale: #c0392b;
  --badge-discount: #c0392b;
  --star-rating: #f1c40f;
  --on-primary: #000;
  --on-dark: #fff;
  --sidebar-bg: #fff8f0;
  --sidebar-border: #fae5d3;
  --sidebar-text: #c0392b;
  --sidebar-muted: #666;
  background: var(--canvas);
  color: var(--ink);
  font-family: Inter, sans-serif;
  overflow: hidden;
}
.container {
  max-width: 1200px;
  padding: 0 15px;
  margin: auto;
}
.ds-hero {
  position: relative;
  overflow: hidden;
  background: var(--surface-soft);
  border-bottom: 1px solid var(--border-color);
}
.hero-inner {
  position: relative;
  z-index: 1;
  padding-top: 88px;
  padding-bottom: 92px;
}
.eyebrow,
.section-kicker,
.component-label {
  color: var(--badge-sale);
  font:
    500 11px/1.2 'DM Mono',
    monospace;
  letter-spacing: 0.08em;
}
.eyebrow {
  display: flex;
  align-items: center;
  gap: 9px;
  margin-bottom: 25px;
}
.eyebrow span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--primary);
  box-shadow: 0 0 0 5px rgba(248, 184, 42, 0.16);
}
h1,
h2,
h3,
p {
  margin: 0;
}
.ds-hero h1 {
  max-width: 650px;
  font-size: clamp(48px, 7vw, 84px);
  line-height: 0.98;
  letter-spacing: -0.065em;
  font-weight: 700;
}
.ds-hero h1 em {
  color: var(--primary);
  font-style: normal;
}
.hero-copy {
  max-width: 490px;
  margin: 28px 0 30px;
  color: var(--muted);
  font-size: 14px;
  line-height: 1.7;
}
.hero-actions,
.button-row,
.badge-row {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}
.version {
  font:
    11px 'DM Mono',
    monospace;
  color: var(--muted);
}
.dot {
  display: inline-block;
  width: 4px;
  height: 4px;
  margin: 0 8px;
  border-radius: 50%;
  background: var(--primary);
  vertical-align: middle;
}
.hero-orbit {
  position: absolute;
  border: 1px solid rgba(248, 184, 42, 0.5);
  border-radius: 50%;
}
.orbit-one {
  width: 540px;
  height: 540px;
  right: -85px;
  top: -180px;
}
.orbit-two {
  width: 360px;
  height: 360px;
  right: 5px;
  top: -90px;
  border-color: rgba(192, 57, 43, 0.2);
}
.hero-sticker {
  position: absolute;
  right: 15%;
  bottom: 62px;
  width: 108px;
  height: 108px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border-radius: 50%;
  background: var(--primary);
  color: var(--on-primary);
  transform: rotate(10deg);
  font-size: 10px;
  font-weight: 700;
  line-height: 1.1;
  text-align: center;
}
.hero-sticker i {
  font-size: 20px;
}
.ds-content {
  padding-bottom: 80px;
}
.intro-grid,
.section-heading,
.responsive-note {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 48px;
}
.intro-grid {
  padding: 76px 0 66px;
  border-bottom: 1px solid var(--border-color);
}
.section-kicker {
  margin-bottom: 16px;
}
.intro-grid h2,
.section-heading h2,
.responsive-note h2 {
  font-size: 32px;
  line-height: 1.15;
  letter-spacing: -0.04em;
}
.intro-text {
  padding-top: 28px;
  color: var(--muted);
  font-size: 14px;
  line-height: 1.7;
}
.principle-list {
  display: flex;
  gap: 24px;
  margin-top: 28px;
  flex-wrap: wrap;
  color: var(--ink);
  font-size: 12px;
}
.principle-list b {
  margin-right: 7px;
  color: var(--primary-hover);
  font:
    11px 'DM Mono',
    monospace;
}
.ds-section {
  padding-top: 72px;
}
.section-heading {
  align-items: end;
  margin-bottom: 28px;
}
.section-heading > p {
  max-width: 320px;
  padding-bottom: 2px;
  color: var(--muted);
  font-size: 12px;
  line-height: 1.6;
}
.section-heading code,
.color-info code,
.type-token code,
.measure-row code,
.radius-items code {
  font:
    11px 'DM Mono',
    monospace;
}
.color-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}
.color-card {
  overflow: hidden;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: #fff;
}
.swatch {
  height: 76px;
}
.color-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
  padding: 12px;
}
.color-info strong {
  font-size: 12px;
  font-weight: 600;
}
.color-info code,
.color-info span {
  color: var(--muted);
  font:
    10px 'DM Mono',
    monospace;
}
.type-section,
.compact-section,
.component-section {
  border-top: 1px solid var(--border-color);
  margin-top: 72px;
}
.type-table {
  border-top: 1px solid var(--border-color);
}
.type-row {
  display: grid;
  grid-template-columns: 170px 1fr;
  align-items: center;
  min-height: 58px;
  gap: 24px;
  border-bottom: 1px solid var(--border-color);
}
.type-token {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.type-token span {
  color: var(--muted);
  font:
    10px 'DM Mono',
    monospace;
}
.type-sample {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}
.geometry-grid,
.component-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}
.token-panel,
.component-card {
  min-width: 0;
  padding: 24px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: #fff;
}
.token-panel h3 {
  margin-bottom: 22px;
  font-size: 16px;
}
.measure-row {
  display: grid;
  grid-template-columns: 72px 1fr 45px;
  align-items: center;
  gap: 10px;
  height: 28px;
}
.measure-row b {
  color: var(--muted);
  font:
    10px 'DM Mono',
    monospace;
  text-align: right;
}
.measure {
  display: block;
  height: 8px;
  max-width: 100%;
  min-width: 2px;
  border-radius: 4px;
  background: var(--primary);
}
.radius-items {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 14px;
}
.radius-items > div {
  display: flex;
  flex-direction: column;
  gap: 11px;
  align-items: center;
  text-align: center;
}
.radius-box {
  display: block;
  width: 58px;
  height: 58px;
  border: 2px solid var(--primary);
  background: rgba(248, 184, 42, 0.14);
}
.radius-box.xs {
  border-radius: 4px;
}
.radius-box.md {
  border-radius: 8px;
}
.radius-box.xl {
  border-radius: 20px;
}
.radius-box.full {
  border-radius: 9999px;
}
.component-card.wide {
  grid-column: 1/-1;
}
.component-label {
  margin-bottom: 20px;
  color: var(--muted);
}
.ds-button {
  border: 1px solid transparent;
  border-radius: 4px;
  padding: 8px 16px;
  font:
    600 14px/1.4 Inter,
    sans-serif;
  cursor: pointer;
  transition:
    background 0.2s,
    transform 0.2s;
}
.ds-button:hover {
  transform: translateY(-1px);
}
.ds-button.primary {
  background: var(--primary);
  color: var(--on-primary);
}
.ds-button.primary:hover {
  background: var(--primary-hover);
}
.ds-button.outline {
  border-color: var(--primary);
  background: transparent;
  color: var(--primary-hover);
}
.ds-button.white {
  border-color: var(--border-color);
  background: #fff;
  color: var(--ink);
}
.ds-button.full {
  width: 100%;
}
.dark-row {
  margin-top: 16px;
  padding: 14px;
  border-radius: 4px;
  background: var(--ink);
}
.icon-button,
.round-button {
  border: 0;
  color: var(--ink);
  background: transparent;
  cursor: pointer;
}
.icon-button {
  font-size: 18px;
  color: var(--on-dark);
}
.round-button {
  width: 34px;
  height: 34px;
  border-radius: 50%;
  background: var(--surface-soft);
}
.search-preview {
  display: flex;
  align-items: center;
  height: 44px;
  overflow: hidden;
  border: 2px solid var(--primary);
  border-radius: 4px;
}
.search-preview span {
  padding: 10px;
  border-right: 1px solid var(--border-color);
  white-space: nowrap;
  color: var(--muted);
  font-size: 11px;
}
.search-preview span i {
  margin-left: 4px;
  font-size: 9px;
}
.search-preview input {
  min-width: 0;
  flex: 1;
  padding: 10px;
  border: 0;
  outline: 0;
  color: var(--ink);
  font:
    13px Inter,
    sans-serif;
}
.search-preview button {
  align-self: stretch;
  width: 44px;
  border: 0;
  border-left: 1px solid var(--border-color);
  background: #fff;
  cursor: pointer;
}
.tab-row {
  display: flex;
  gap: 13px;
  overflow: auto;
  padding-top: 22px;
  white-space: nowrap;
}
.tab-row button {
  padding: 0;
  border: 0;
  background: transparent;
  color: var(--muted);
  cursor: pointer;
  font:
    13px Inter,
    sans-serif;
}
.tab-row button.active {
  color: var(--ink);
  font-weight: 700;
}
.product-demo {
  grid-row: span 2;
}
.product-card {
  position: relative;
  display: flex;
  flex-direction: column;
  padding: 15px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: #fff;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.08);
}
.badge {
  display: inline-flex;
  width: max-content;
  padding: 3px 8px;
  border-radius: 4px;
  color: #fff;
  font:
    700 11px/1.2 Inter,
    sans-serif;
}
.product-card > .badge {
  position: absolute;
  top: 25px;
  left: 25px;
  z-index: 1;
}
.badge.sale,
.badge.discount {
  background: var(--badge-sale);
}
.product-image {
  height: 160px;
  margin-bottom: 15px;
  overflow: hidden;
  border-radius: 4px;
  background: var(--surface-soft);
}
.product-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s;
}
.product-card:hover .product-image img {
  transform: scale(1.05);
}
.product-brand {
  margin-bottom: 5px;
  color: var(--muted);
  font-size: 11px;
}
.product-card h3 {
  min-height: 39px;
  margin-bottom: 10px;
  font-size: 14px;
  font-weight: 600;
  line-height: 1.4;
}
.rating {
  margin-bottom: 10px;
  color: var(--star-rating);
  font-size: 11px;
}
.rating i {
  color: var(--border-color);
  font-style: normal;
}
.rating small {
  margin-left: 5px;
  color: var(--muted);
}
.price {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 15px;
}
.price del {
  color: var(--muted);
  font:
    12px/1.3 Inter,
    sans-serif;
}
.price strong {
  color: var(--badge-sale);
  font-size: 16px;
}
.qty-row {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}
.qty-box {
  display: flex;
  overflow: hidden;
  border: 1px solid var(--border-color);
  border-radius: 4px;
}
.qty-box button {
  width: 31px;
  height: 30px;
  border: 0;
  background: var(--surface-soft);
  cursor: pointer;
}
.qty-box output {
  display: grid;
  width: 30px;
  place-items: center;
  border-right: 1px solid var(--border-color);
  border-left: 1px solid var(--border-color);
  font:
    12px 'DM Mono',
    monospace;
}
.qty-row > span {
  color: var(--muted);
  font-size: 11px;
}
.category-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}
.category-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 16px 8px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  text-align: center;
}
.category-card.active {
  border-color: transparent;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}
.category-card span {
  display: grid;
  width: 52px;
  height: 52px;
  place-items: center;
  border-radius: 50%;
  background: var(--surface-soft);
  color: var(--primary-hover);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}
.category-card b {
  font-size: 11px;
  line-height: 1.3;
}
.badge-row {
  margin-top: 22px;
}
.stars {
  color: var(--star-rating);
  font-size: 14px;
  letter-spacing: 2px;
}
.sidebar-widget {
  padding: 30px 20px;
  border: 1px solid var(--sidebar-border);
  border-radius: 8px;
  background: var(--sidebar-bg);
  text-align: center;
}
.sidebar-spark {
  color: var(--primary);
  font-size: 22px;
}
.sidebar-widget h3 {
  margin: 7px 0 8px;
  color: var(--sidebar-text);
  font-size: 24px;
}
.sidebar-widget p {
  margin-bottom: 18px;
  color: var(--sidebar-muted);
  font-size: 13px;
  line-height: 1.5;
}
.sidebar-widget input {
  width: 100%;
  margin-bottom: 10px;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  outline: 0;
  font:
    14px Inter,
    sans-serif;
}
.dialog-mini {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1);
}
.dialog-mini-head,
.dialog-mini-body,
.dialog-mini-foot {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
  padding: 16px;
}
.dialog-mini-head {
  border-bottom: 1px solid var(--border-color);
}
.dialog-mini-head h3 {
  font-size: 16px;
  font-weight: 700;
}
.dialog-mini-head p {
  margin-top: 4px;
  color: var(--muted);
  font-size: 11px;
}
.dialog-mini-head > i {
  color: var(--muted);
}
.dialog-mini-body {
  align-items: center;
  justify-content: flex-start;
}
.mini-product {
  display: grid;
  width: 48px;
  height: 48px;
  place-items: center;
  border-radius: 4px;
  background: var(--surface-soft);
  color: var(--primary-hover);
}
.dialog-mini-body div:last-child {
  display: flex;
  flex-direction: column;
  gap: 5px;
  font-size: 12px;
}
.dialog-mini-body span {
  color: var(--badge-sale);
  font-weight: 700;
}
.dialog-mini-foot {
  justify-content: flex-end;
  border-top: 1px solid var(--border-color);
}
.dialog-copy {
  color: var(--muted);
  font-size: 14px;
  line-height: 1.7;
}
.responsive-note {
  align-items: center;
  margin-top: 72px;
  padding: 48px 0;
  border-top: 1px solid var(--border-color);
  border-bottom: 1px solid var(--border-color);
}
.breakpoints {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}
.breakpoints > div {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 18px 14px;
  border-left: 3px solid var(--primary);
  background: var(--surface-soft);
}
.breakpoints b {
  font-size: 14px;
}
.breakpoints span {
  color: var(--primary-hover);
  font:
    11px 'DM Mono',
    monospace;
}
.breakpoints small {
  color: var(--muted);
  font-size: 11px;
  line-height: 1.5;
}
@media (max-width: 900px) {
  .color-grid {
    grid-template-columns: repeat(3, 1fr);
  }
  .hero-sticker {
    right: 8%;
  }
  .component-grid {
    grid-template-columns: 1fr;
  }
  .product-demo {
    grid-row: auto;
  }
}
@media (max-width: 767px) {
  .hero-inner {
    padding-top: 58px;
    padding-bottom: 68px;
  }
  .ds-hero h1 {
    font-size: 54px;
  }
  .hero-sticker {
    right: 18px;
    bottom: 30px;
    transform: scale(0.75) rotate(10deg);
    transform-origin: bottom right;
  }
  .hero-copy {
    max-width: 390px;
  }
  .hero-actions {
    align-items: flex-start;
    flex-direction: column;
  }
  .intro-grid,
  .section-heading,
  .responsive-note {
    grid-template-columns: 1fr;
    gap: 18px;
  }
  .intro-text {
    padding-top: 0;
  }
  .intro-grid h2,
  .section-heading h2,
  .responsive-note h2 {
    font-size: 27px;
  }
  .ds-section,
  .type-section,
  .compact-section,
  .component-section {
    padding-top: 54px;
    margin-top: 54px;
  }
  .color-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  .type-row {
    grid-template-columns: 105px 1fr;
    gap: 12px;
    min-height: 68px;
  }
  .type-sample {
    font-size: 14px !important;
  }
  .geometry-grid {
    grid-template-columns: 1fr;
  }
  .radius-items {
    gap: 8px;
  }
  .radius-box {
    width: 48px;
    height: 48px;
  }
  .component-card {
    padding: 18px;
  }
  .search-preview span {
    display: none;
  }
  .breakpoints {
    grid-template-columns: 1fr;
  }
  .responsive-note {
    margin-top: 54px;
    padding: 42px 0;
  }
}
.dialog-live {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 18px;
  flex-wrap: wrap;
}
.dialog-live span {
  color: var(--muted);
  font:
    10px 'DM Mono',
    monospace;
}
</style>

# Farmart — ร้านขายของชำออนไลน์

## เป้าหมายธุรกิจ (Business Goals)
- เพิ่มยอดขายออนไลน์
- กระตุ้นให้ลูกค้ากลับมาซื้อซ้ำผ่านระบบสมาชิก (Member)
- ลดภาระงานหน้าร้าน

---

## Phase 1: Product Discovery & Browsing (ค้นหาและเรียกดูสินค้า)

### Category Navigation (การนำทางหมวดหมู่)
- [ ] กำหนดหมวดหมู่สินค้า: ผัก, ผลไม้, เนื้อสัตว์, นมและผลิตภัณฑ์, เครื่องดื่ม, ของใช้ในครัว
- [ ] สร้าง Sidebar/Header bar แสดงหมวดหมู่บนหน้าแรก
- [ ] แสดง Breadcrumb + Filter หมวดหมู่บนหน้ารายการสินค้า
- [ ] ทำระบบค้นหาสินค้าพร้อมกรองตาม keyword และหมวดหมู่

### Product Listing (รายการสินค้า)
- [ ] แสดงราคาสินค้า (Price)
- [ ] แสดงส่วนลด (Discount %) ถ้ามี
- [ ] แสดงคะแนนรีวิว (Rating: stars + score)
- [ ] มีปุ่ม "หยิบใส่ตะกร้า" (Add to Cart) บน Card สินค้า
- [ ] แสดงรูปภาพสินค้าความละเอียดสูง
- [ ] แสดงชื่อและน้ำหนัก/ปริมาณสินค้า

### Homepage (หน้าแรก)
- [ ] ออกแบบ Hero Section โชว์โปรโมชัน/ของลดราคาดึงดูด
- [ ] วาง Category cards/pills ให้เข้าถึงหมวดหมู่หลักได้ง่าย
- [ ] สร้างโซน "Best Seller" แสดงสินค้าขายดี
- [ ] สร้างโซน "Top Saver" แสดงสินค้าประหยัดสูงสุด / ลดราคาแรง

---

## Phase 2: Shopping Cart & Checkout (ตะกร้าสินค้าและชำระเงิน)

### Cart Management (จัดการตะกร้า)
- [x] เพิ่มสินค้าเข้าตะกร้า
- [x] ลบสินค้าออกจากตะกร้า
- [x] แก้ไขจำนวนสินค้าในตะกร้า
- [x] แสดงยอดรวม (Subtotal) และยอดรวมทั้งสิ้น (Grand Total)
- [ ] แสดงส่วนลดที่ได้รับ (ถ้ามี)
- [x] แสดงตะกร้าสินค้าใน Header/Navbar ทุกหน้า (Cart icon + count)

### Checkout Flow (ขั้นตอนชำระเงิน)
- [ ] หน้า Review ตะกร้าก่อนสั่งซื้อ
- [ ] เลือกที่อยู่จัดส่ง / แก้ไขที่อยู่
- [ ] เลือกวิธีการชำระเงิน (COD, Credit Card, PromptPay)
- [ ] ยืนยันคำสั่งซื้อ
- [ ] แสดง Order confirmation + เลขคำสั่งซื้อ

---

## Phase 3: Membership & Engagement (ระบบสมาชิกและการกระตุ้นลูกค้า)

### Member Registration (สมัครสมาชิก)
- [ ] ฟอร์มสมัครสมาชิก (ชื่อ, Email, Password, เบอร์โทร)
- [ ] ระบบ Login/Logout
- [ ] หน้าโปรโมชัน "สมัครสมาชิกใหม่รับส่วนลด 15%" (New Member 15% off)

### Member Features (ฟีเจอร์สำหรับสมาชิก)
- [ ] แสดงส่วนลดสมาชิกในราคาสินค้า
- [ ] หน้าประวัติการสั่งซื้อ (Order History)
- [ ] ระบบ Point/Reward (ถ้ามี)
- [ ] โปรไฟล์สมาชิก: แก้ไขข้อมูลส่วนตัว, ที่อยู่จัดส่ง

---

## Phase 4: Admin / Backend (ระบบหลังบ้าน)

### Product Management (จัดการสินค้า)
- [ ] CRUD สินค้า (สร้าง, แก้ไข, ลบ, ดู)
- [ ] กำหนดหมวดหมู่สินค้า
- [ ] ตั้งราคา ส่วนลด และ stock
- [ ] อัปโหลดรูปสินค้า

### Promotion Management (จัดการโปรโมชัน)
- [ ] สร้าง/แก้ไข Banner โปรโมชันหน้าแรก
- [ ] กำหนดส่วนลดสมาชิกใหม่ (เช่น 15%)
- [ ] กำหนดสินค้า Best Seller / Top Saver

### Order Management (จัดการคำสั่งซื้อ)
- [ ] ดูรายการคำสั่งซื้อทั้งหมด
- [ ] อัปเดตสถานะคำสั่งซื้อ (Pending → Confirmed → Shipped → Delivered)
- [ ] ดูรายละเอียดคำสั่งซื้อแต่ละรายการ

---

## Phase 5: UX/UI & Polish (ปรับปรุงประสบการณ์ผู้ใช้)

### Responsive Design
- [ ] รองรับ Mobile, Tablet, Desktop
- [ ] ทดสอบ UI บน Browser หลัก (Chrome, Safari, Firefox)

### Performance & SEO
- [ ] Image lazy loading สำหรับหน้ารายการสินค้า
- [ ] เพิ่ม Meta tags, OG tags สำหรับ Social sharing
- [ ] ทำ Search engine friendly URLs

### Empty & Error States
- [ ] หน้าค้นหาไม่พบสินค้า (Empty search result)
- [ ] หน้าตะกร้าว่าง (Empty cart)
- [ ] หน้า Error 404/500 ที่สวยงาม


# การใช้ Graphify ในโปรเจกต์ Farmart

Graphify ใช้สร้างและค้น knowledge graph ของ repository เพื่อช่วยทำความเข้าใจโครงสร้างระบบ ความสัมพันธ์ระหว่างไฟล์ และเส้นทางการไหลของข้อมูล ก่อนแก้ไขโค้ดควรใช้ Graphify สำรวจผลกระทบของการเปลี่ยนแปลงก่อนเสมอ

## ขอบเขตของโปรเจกต์

Farmart แบ่งเป็นสองส่วนหลัก:

- `vue-app/` — Vue 3 + TypeScript SPA
- `backend/` — Rust Axum API + SQLite

จุดเชื่อมต่อสำคัญที่ควรใช้เป็นคำค้น ได้แก่ `src/services/api.ts`, `src/router/`, `src/views/`, Axum routes, DTOs, `CatalogPort`, และ Diesel adapter ใน `backend/src/adapters/outbound/sqlite.rs`

## เริ่มต้นใช้งาน

1. ตรวจสอบว่า repository มีผลลัพธ์ Graphify อยู่ใน `graphify-out/`
2. หากยังไม่มี ให้รันขั้นตอน ingest/index ตามคำสั่งของ Graphify ที่ติดตั้งในสภาพแวดล้อมนั้น
3. เริ่มจาก query ภาพรวมก่อนเจาะจงไฟล์หรือฟังก์ชัน

ผลลัพธ์ที่สร้างโดย Graphify เป็นข้อมูลช่วยวิเคราะห์ ไม่ใช่ source code ที่ต้องแก้ไขด้วยมือ

## รูปแบบคำถามที่แนะนำ

### ดูภาพรวมระบบ

ใช้เพื่อทำความเข้าใจโมดูลและจุดเชื่อมต่อ:

```text
อธิบายสถาปัตยกรรมของ Farmart และความสัมพันธ์ระหว่าง vue-app กับ backend
```

```text
แสดงโมดูลหลักของ backend และ frontend พร้อมหน้าที่ของแต่ละโมดูล
```

### ตามเส้นทางการทำงาน

ใช้ `path` เมื่ออยากรู้ว่าข้อมูลเดินทางจากต้นทางไปปลายทางอย่างไร:

```text
หาเส้นทางจาก Product Detail view ไปยัง GET /api/v1/products/{id}/detail
```

```text
ตามเส้นทางจาก product filter ในหน้า frontend ไปยัง SQLite adapter
```

```text
แสดงเส้นทางของการ register ตั้งแต่ form, API client, route handler จนถึง validation
```

### อธิบายไฟล์หรือสัญลักษณ์

ใช้ `explain` เมื่ออยากรู้บทบาทและผู้ใช้ของไฟล์ ฟังก์ชัน หรือ type:

```text
อธิบาย vue-app/src/services/api.ts และบอกว่าไฟล์ใดเรียกใช้บ้าง
```

```text
อธิบาย CatalogPort, implementation ของมัน และ route ที่พึ่งพา port นี้
```

```text
อธิบายความสัมพันธ์ระหว่าง DTO, Utoipa annotation และ /openapi.json
```

### ตรวจผลกระทบก่อนแก้ไข

```text
ถ้าเปลี่ยน Product DTO ต้องตรวจสอบไฟล์ route, API client, views, tests และ OpenAPI ส่วนใดบ้าง
```

```text
ถ้าเพิ่ม query parameter ใน GET /products มี consumer หรือ test ใดได้รับผลกระทบ
```

## Workflow ที่ควรใช้กับงานพัฒนา

1. Query Graphify เพื่อระบุจุดเริ่มต้นและ dependency ที่เกี่ยวข้อง
2. อ่าน source code จริงของไฟล์ที่ Graphify พบ
3. ตรวจ `DESIGN.md` และ `PLAN.md` เพื่อยืนยันกติกาด้าน UI และ scope
4. แก้ไขโค้ดให้ครบทั้งชั้นที่เกี่ยวข้อง เช่น DTO, route, client และ test
5. Query Graphify ซ้ำเพื่อตรวจว่าเส้นทางและความสัมพันธ์ยังถูกต้อง
6. รัน verification ตามคำสั่งใน `AGENTS.md`

## ตัวอย่างงานเพิ่ม endpoint

สมมติจะเพิ่ม `GET /brands/{id}` ให้ถาม Graphify ตามลำดับ:

```text
แสดงโครงสร้างของ endpoint GET /brands และเส้นทางไปยัง repository
```

```text
จุดใดใน backend ที่ต้องแก้เพื่อเพิ่ม GET /brands/{id} ให้สอดคล้องกับรูปแบบเดิม
```

```text
ตรวจผลกระทบของการเพิ่ม Brand detail DTO ต่อ Utoipa schema, frontend API client และ tests
```

จากนั้นจึงตรวจ source code จริงและทำตาม convention ที่มีอยู่ อย่าถือว่า item ที่ยังไม่ implement ใน `PLAN.md` เป็นความสามารถที่มีอยู่แล้ว

## ข้อควรระวัง

- ใช้ Graphify เพื่อค้นหาและวิเคราะห์ ไม่ใช่แทนการอ่าน source code
- แยกสิ่งที่ Graphify อนุมานออกจากสิ่งที่ตรวจพบในไฟล์จริง
- เมื่อ API เปลี่ยน ต้องตรวจ DTO, route annotation, generated OpenAPI และเอกสารพร้อมกัน
- เมื่อ UI เปลี่ยน ต้องตรวจ router, view, reusable component, API client และ tests
- ไม่แก้ไฟล์ใน `graphify-out/` เว้นแต่ workflow ของ Graphify ระบุไว้อย่างชัดเจน

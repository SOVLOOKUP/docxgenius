
import { DocxTemplate } from "../index.js"
import fs from "fs"

const data = fs.readFileSync("test/assets/data.json", "utf-8")
const template = fs.readFileSync("test/assets/template.docx")
const dt = new DocxTemplate()

// 1
dt.renderFile("test/assets/template.docx", "target/out.docx", data)

// 2
const out = dt.renderBase64(template.toString("base64"), data)
fs.writeFileSync("target/out2.docx", Buffer.from(out, "base64"))

// 3
const out2 = dt.templateMeta(template.toString("base64"))

console.log(out2)
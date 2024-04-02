
import { DocxTemplate } from "../index.js"
import fs from "fs"

const data = fs.readFileSync("test/assets/data.json", "utf-8")
const dt = new DocxTemplate()

dt.renderFile("test/assets/template.docx", "target/out.docx", data)

const template = fs.readFileSync("test/assets/template.docx")

const out = dt.renderBase64(template.toString("base64"), data)

fs.writeFileSync("target/out2.docx", Buffer.from(out, "base64"))

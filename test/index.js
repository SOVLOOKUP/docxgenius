
import { DocxTemplate } from "../index.js"
import fs from "fs"
const data = fs.readFileSync("test/assets/data.json", "utf-8")

const dt = new DocxTemplate()

dt.renderFile("test/assets/template.docx", "target/out.docx", data)
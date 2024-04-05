import { expect, test } from 'vitest'
import { DocxTemplate } from '../index'
import fs from "fs"

const data = fs.readFileSync("test/assets/data.json", "utf-8")
const template = fs.readFileSync("test/assets/template.docx")
const dt = new DocxTemplate()

test('Render Docx File', () => {
    dt.renderFile("test/assets/template.docx", "target/out.docx", data)
    expect(fs.existsSync("target/out.docx")).toBe(true)
})

test('Render Docx Base64', () => {
    const out = dt.renderBase64(template.toString("base64"), data)
    expect(typeof out).toBe("string")
})

test('Render Docx MetaData', () => {
    const out = dt.templateMeta(template.toString("base64"))
    expect(out.length).toBe(11)
})

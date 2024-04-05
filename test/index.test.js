const { DocxTemplate } = require("..")
const assert = require("assert")
const fs = require("fs")

const data = fs.readFileSync("test/assets/data.json", "utf-8")
const template = fs.readFileSync("test/assets/template.docx")
const dt = new DocxTemplate()

it('Render Docx File', () => {
    dt.renderFile("test/assets/template.docx", "target/out.docx", data)
    assert.strictEqual(fs.existsSync("target/out.docx"), true)
}).timeout(10000)

it('Render Docx Base64', () => {
    const out = dt.renderBase64(template.toString("base64"), data)
    assert.strictEqual(typeof out, "string")
})

it('Render Docx MetaData', () => {
    const out = dt.templateMeta(template.toString("base64"))
    assert.strictEqual(out.length, 11)
})

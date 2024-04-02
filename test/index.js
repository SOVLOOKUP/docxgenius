
import { DocxTemplate } from "../index.js"

const dt = new DocxTemplate()

dt.renderFile("C:\\Users\\xiafan\\Desktop\\story.docx", "C:\\Users\\xiafan\\Desktop\\new_story.docx", JSON.stringify(
    {
        story_name: "故事名称",
        story_author: "作者",
        story_source: "来源"
    }
))
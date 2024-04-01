
import { DocxTemplate } from "../index.js"

const dt = DocxTemplate()

dt.renderFile("C:\\Users\\xiafan\\Desktop\\story.docx", "C:\\Users\\xiafan\\Desktop\\new_story.docx", JSON.stringify(
    {
        storyName: "故事名称",
        storyAuthor: "作者",
        storySource: "来源"
    }
))
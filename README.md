# docxtemplate

[poi-tl](https://github.com/Sayi/poi-tl)-based docx template generator

基于 [poi-tl](https://github.com/Sayi/poi-tl) 的 docx 模板生成器

```
pnpm add docxtemplate
```

```ts
export class DocxTemplate {
  constructor()
  // 从模板文件渲染  
  renderFile(tplPath: string, outPath: string, jsonData: string): void
  // 从 base64 渲染  
  renderBase64(template: string, jsonData: string): string
  // 获取模板文件中的变量  
  templateMeta(template: string): Array<string>
}
```

## 支持平台 | Supported Platforms

 - x86_64-apple-darwin
 - aarch64-apple-darwin
 - aarch64-linux-android
 - aarch64-pc-windows-msvc
 - armv7-unknown-linux-gnueabihf
 - i686-pc-windows-msvc
 - armv7-linux-androideabi
 - x86_64-pc-windows-msvc
 - x86_64-unknown-linux-gnu

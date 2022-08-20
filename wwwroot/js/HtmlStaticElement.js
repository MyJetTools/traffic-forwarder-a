class HtmlStaticElement {
    constructor(el, renderHtml) {
        this.el = el;
        this.renderHtml = renderHtml;
    }
    update(value) {
        if (this.value === undefined || this.value != value) {
            this.value = value;
            this.el.innerHTML = this.renderHtml(value);
        }
    }
}
//# sourceMappingURL=HtmlStaticElement.js.map
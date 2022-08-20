class main {
    static init() {
        this.body = document.getElementsByTagName('body')[0];
        this.body.innerHTML = HtmlMain.layout();
        this.layoutElement = document.getElementById('main');
        this.statusBarElement = document.getElementById('status-bar');
        HtmlStatusBar.init();
    }
    static generatePosition(left, top, width, height) {
        return 'top:' + top + 'px; left:' + left + 'px; width:' + width + 'px; height:' + height + 'px';
    }
    static resize() {
        let height = window.innerHeight;
        let width = window.innerWidth;
        if (this.windowHeight == height && this.windowWidth == width)
            return;
        this.windowHeight = height;
        this.windowWidth = width;
        let sbHeight = this.statusBarHeight;
        this.layoutElement.setAttribute('style', this.generatePosition(0, 0, width, height - sbHeight));
        this.statusBarElement.setAttribute('style', 'position:absolute; ' + this.generatePosition(0, height - sbHeight, width, sbHeight));
    }
    static background() {
        this.resize();
        if (this.requested)
            return;
        this.requested = true;
        $.ajax({ url: '/api/status', type: 'get' })
            .then((result) => {
            this.requested = false;
            HtmlStatusBar.updateStatusbar(result);
        }).fail(() => {
            this.requested = false;
            HtmlStatusBar.updateOffline();
        });
    }
}
main.requested = false;
main.statusBarHeight = 24;
window.setTimeout(() => {
    main.init();
    main.background();
}, 100);
window.setInterval(() => main.background(), 1000);
//# sourceMappingURL=main.js.map
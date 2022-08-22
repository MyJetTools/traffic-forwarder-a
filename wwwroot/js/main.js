var main = /** @class */ (function () {
    function main() {
    }
    main.init = function () {
        this.body = document.getElementsByTagName('body')[0];
        this.body.innerHTML = HtmlMain.layout();
        this.layoutElement = document.getElementById('main');
        this.statusBarElement = document.getElementById('status-bar');
        HtmlStatusBar.init();
    };
    main.generatePosition = function (left, top, width, height) {
        return 'top:' + top + 'px; left:' + left + 'px; width:' + width + 'px; height:' + height + 'px';
    };
    main.resize = function () {
        var height = window.innerHeight;
        var width = window.innerWidth;
        if (this.windowHeight == height && this.windowWidth == width)
            return;
        this.windowHeight = height;
        this.windowWidth = width;
        var sbHeight = this.statusBarHeight;
        this.layoutElement.setAttribute('style', this.generatePosition(0, 0, width, height - sbHeight));
        this.statusBarElement.setAttribute('style', 'position:absolute; ' + this.generatePosition(0, height - sbHeight, width, sbHeight));
    };
    main.background = function () {
        var _this = this;
        this.resize();
        if (this.requested)
            return;
        this.requested = true;
        $.ajax({ url: '/api/status', type: 'get' })
            .then(function (response) {
            _this.requested = false;
            _this.layoutElement.innerHTML = HtmlTunnelTraffic.render(response)
                + HtmlServices.generate(response);
            HtmlStatusBar.updateStatusbar(response);
        }).fail(function () {
            _this.requested = false;
            HtmlStatusBar.updateOffline();
        });
    };
    main.requested = false;
    main.statusBarHeight = 24;
    return main;
}());
window.setTimeout(function () {
    main.init();
    main.background();
}, 100);
window.setInterval(function () { return main.background(); }, 1000);
//# sourceMappingURL=main.js.map
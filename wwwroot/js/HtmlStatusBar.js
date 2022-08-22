var HtmlStatusBar = /** @class */ (function () {
    function HtmlStatusBar() {
    }
    HtmlStatusBar.layout = function () {
        return '<div id="status-bar">' +
            '<table><tr>' +
            '<td style="padding-left: 5px">Online: <b id="online" style="text-shadow: 0 0 2px white;">N/A</b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '<td style="padding-left: 5px">Tunnel connected: <b id="tunnel-connected" style="text-shadow: 0 0 2px white;">N/A</b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '</tr></table></div>';
    };
    HtmlStatusBar.renderBool = function (value) {
        return value ? '<b style="color:green">Yes</b>' : '<b style="color:red">No</b>';
    };
    HtmlStatusBar.init = function () {
        this.htmlOnline = new HtmlStaticElement(document.getElementById('online'), this.renderBool);
        this.htmlTunnelConnected = new HtmlStaticElement(document.getElementById('tunnel-connected'), this.renderBool);
    };
    HtmlStatusBar.updateStatusbar = function (statusBar) {
        this.htmlOnline.update(true);
        this.htmlTunnelConnected.update(statusBar.tunnelConnected);
    };
    HtmlStatusBar.updateOffline = function () {
        this.htmlOnline.update(false);
        this.htmlTunnelConnected.update(false);
    };
    return HtmlStatusBar;
}());
//# sourceMappingURL=HtmlStatusBar.js.map
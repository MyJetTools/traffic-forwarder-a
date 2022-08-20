class HtmlStatusBar {
    static layout() {
        return '<div id="status-bar">' +
            '<table><tr>' +
            '<td style="padding-left: 5px">Online: <b id="online" style="text-shadow: 0 0 2px white;">N/A</b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '<td style="padding-left: 5px">Tunnel connected: <b id="tunnel-connected" style="text-shadow: 0 0 2px white;">N/A</b></td>' +
            '<td><div class="statusbar-separator"></div></td>' +
            '</tr></table></div>';
    }
    static renderBool(value) {
        return value ? '<b style="color:green">Yes</b>' : '<b style="color:red">No</b>';
    }
    static init() {
        this.htmlOnline = new HtmlStaticElement(document.getElementById('online'), this.renderBool);
        this.htmlTunnelConnected = new HtmlStaticElement(document.getElementById('tunnel-connected'), this.renderBool);
    }
    static updateStatusbar(statusBar) {
        this.htmlOnline.update(true);
        this.htmlTunnelConnected.update(statusBar.tunnelConnected);
    }
    static updateOffline() {
        this.htmlOnline.update(false);
        this.htmlTunnelConnected.update(false);
    }
}
//# sourceMappingURL=HtmlStatusBar.js.map
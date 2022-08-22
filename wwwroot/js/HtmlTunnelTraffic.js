var HtmlTunnelTraffic = /** @class */ (function () {
    function HtmlTunnelTraffic() {
    }
    HtmlTunnelTraffic.render = function (status) {
        return "<h1>Tunnel traffic</h1>" + HtmlGraph.renderGraph(status.tunnelTrafficHistory, function (n) { return Utils.formatBytes(n); });
    };
    return HtmlTunnelTraffic;
}());
//# sourceMappingURL=HtmlTunnelTraffic.js.map
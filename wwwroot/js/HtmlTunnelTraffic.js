class HtmlTunnelTraffic {
    static render(status) {
        return "<h1>Tunnel traffic</h1>" + HtmlGraph.renderGraph(status.tunnelTrafficHistory, (n) => Utils.formatBytes(n));
    }
}
//# sourceMappingURL=HtmlTunnelTraffic.js.map
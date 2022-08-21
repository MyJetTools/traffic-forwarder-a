class HtmlTunnelTraffic {

    public static render(status: IStatusContract): string {

        return "<h1>Tunnel traffic</h1>" + HtmlGraph.renderGraph(status.tunnelTrafficHistory, (n) => Utils.formatBytes(n));

    }

}
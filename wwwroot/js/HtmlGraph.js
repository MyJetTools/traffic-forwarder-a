var HtmlGraph = /** @class */ (function () {
    function HtmlGraph() {
    }
    HtmlGraph.getMax = function (traffics) {
        if (traffics.length == 0)
            return 0;
        var max = traffics[0].i;
        for (var _i = 0, traffics_1 = traffics; _i < traffics_1.length; _i++) {
            var itm = traffics_1[_i];
            if (itm.i > max) {
                max = itm.i;
            }
            if (itm.o > max) {
                max = itm.o;
            }
        }
        return max;
    };
    HtmlGraph.renderGraph = function (traffics, showValue) {
        var max = this.getMax(traffics);
        var w = 50;
        var coef = max == 0 ? 0 : w / max;
        var result = '<svg style="font-size:16px" width="480" height="' +
            w +
            '"> <rect width="480" height="' +
            w +
            '" style="fill:none;stroke-width:;stroke:black" />';
        var i = 0;
        for (var _i = 0, traffics_2 = traffics; _i < traffics_2.length; _i++) {
            var m = traffics_2[_i];
            var y = w - m.o * coef;
            result +=
                '<line x1="' +
                    i +
                    '" y1="' +
                    w +
                    '" x2="' +
                    i +
                    '" y2="' +
                    y +
                    '" style="stroke:green;stroke-width:2" />';
            y = w - m.i * coef;
            i += 2;
            result +=
                '<line x1="' +
                    i +
                    '" y1="' +
                    w +
                    '" x2="' +
                    i +
                    '" y2="' +
                    y +
                    '" style="stroke:blue;stroke-width:2" />';
            i += 2;
        }
        var maxValue = showValue(max);
        return result + '<text x="1" y="16" fill="black">' + maxValue + '</text><text x="0" y="15" fill="lime">' + maxValue + '</text></svg>';
    };
    return HtmlGraph;
}());
//# sourceMappingURL=HtmlGraph.js.map
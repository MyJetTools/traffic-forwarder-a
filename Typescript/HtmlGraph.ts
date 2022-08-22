class HtmlGraph {

    static getMax(traffics: ITrafficMoment[]): number {
        if (traffics.length == 0)
            return 0;

        let max = traffics[0].i;

        for (let itm of traffics) {
            if (itm.i > max) {
                max = itm.i;
            }

            if (itm.o > max) {
                max = itm.o;
            }
        }

        return max;
    }

    public static renderGraph(traffics: ITrafficMoment[], showValue: (number) => string) {
        const max = this.getMax(traffics);

        const w = 50;

        let coef = max == 0 ? 0 : w / max;

        let result =
            '<svg style="font-size:16px" width="480" height="' +
            w +
            '"> <rect width="480" height="' +
            w +
            '" style="fill:none;stroke-width:;stroke:black" />';

        let i = 0;
        for (let m of traffics) {
            let y = w - m.o * coef;

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

        let maxValue = showValue(max);
        return result + '<text x="1" y="16" fill="black">' + maxValue + '</text><text x="0" y="15" fill="lime">' + maxValue + '</text></svg>';
    }

}
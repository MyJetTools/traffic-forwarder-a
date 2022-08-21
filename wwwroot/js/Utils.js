class Utils {
    static filterIt(line, filterPhrase) {
        if (filterPhrase == "")
            return false;
        return line.indexOf(filterPhrase) == -1;
    }
    static copyToClipboardHtml(text) {
        return ' style="cursor:pointer" clipboard=' + text + ' onclick="Utils.copyToClipboard()"';
    }
    static copyToClipboard(el) {
        let attr = el.attributes.getNamedItem('clipboard');
        if (attr) {
            navigator.clipboard.writeText(attr.value);
        }
    }
    static getMax(c) {
        let result = 0;
        for (const i of c) {
            if (i > result)
                result = i;
        }
        return result;
    }
    static formatNumber(n) {
        return n.toString().replace(/(\d)(?=(\d{3})+(?!\d))/g, '$1,');
    }
    static formatBytes(n) {
        if (n < 1024) {
            return n.toFixed(2) + "b";
        }
        n = n / 1024;
        if (n < 1024) {
            return n.toFixed(2) + "Kb";
        }
        n = n / 1024;
        if (n < 1024) {
            return n.toFixed(2) + "Mb";
        }
        n = n / 1024;
        return n.toFixed(2) + "Gb";
    }
    static format_duration(micros) {
        if (micros == 0)
            return "0";
        if (micros < 1000) {
            return micros + "µs";
        }
        if (micros < 1000000) {
            return (micros / 1000).toFixed(3) + "ms";
        }
        return (micros / 1000000).toFixed(3) + "s";
    }
}
//# sourceMappingURL=Utils.js.map
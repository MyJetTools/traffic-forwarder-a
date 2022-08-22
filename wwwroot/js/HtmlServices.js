var HtmlServices = /** @class */ (function () {
    function HtmlServices() {
    }
    HtmlServices.generate = function (data) {
        var result = "<table class=\"table\"><tr><th>Port</th><th>Remote host</th><th>Connections</th></tr>";
        for (var _i = 0, _a = data.services; _i < _a.length; _i++) {
            var itm = _a[_i];
            result += "<tr><td>" + itm.port + "</td><td>" + itm.remoteHost + "</td><td>" + itm.connections + "</td></td>";
        }
        return result + "</table>";
    };
    return HtmlServices;
}());
//# sourceMappingURL=HtmlServices.js.map
class HtmlServices {

    public static generate(data: IStatusContract): string {
        let result = `<table class="table"><tr><th>Port</th><th>Remote host</th><th>Connections</th></tr>`;


        for (let itm of data.services) {
            result += `<tr><td>` + itm.port + `</td><td>` + itm.remoteHost + `</td><td>` + itm.connections + `</td></td>`;
        }

        return result + `</table>`;
    }
}

class main {
    private static body: HTMLElement;
    private static layoutElement: HTMLElement;
    private static statusBarElement: HTMLElement;
    private static requested: boolean = false;

    private static windowHeight: number;
    private static windowWidth: number;

    private static statusBarHeight = 24;


    static init() {
        this.body = document.getElementsByTagName('body')[0];
        this.body.innerHTML = HtmlMain.layout();
        this.layoutElement = document.getElementById('main');
        this.statusBarElement = document.getElementById('status-bar');
        HtmlStatusBar.init();

    }

    static generatePosition(left: number, top: number, width: number, height: number): string {
        return 'top:' + top + 'px; left:' + left + 'px; width:' + width + 'px; height:' + height + 'px';
    }


    static resize() {

        let height = window.innerHeight;
        let width = window.innerWidth;


        if (this.windowHeight == height && this.windowWidth == width)
            return;

        this.windowHeight = height;
        this.windowWidth = width;

        let sbHeight = this.statusBarHeight;

        this.layoutElement.setAttribute('style',
            this.generatePosition(0, 0, width, height - sbHeight));

        this.statusBarElement.setAttribute('style',
            'position:absolute; ' + this.generatePosition(0, height - sbHeight, width, sbHeight))

    }

    static background() {

        this.resize();

        if (this.requested)
            return;


        this.requested = true;

        $.ajax({ url: '/api/status', type: 'get' })
            .then((response: IStatusContract) => {
                this.requested = false;

                this.layoutElement.innerHTML = HtmlTunnelTraffic.render(response)
                    + HtmlServices.generate(response);


                HtmlStatusBar.updateStatusbar(response);

            }).fail(() => {
                this.requested = false;
                HtmlStatusBar.updateOffline();
            })

    }
}

window.setTimeout(() => {
    main.init();
    main.background();
}, 100);


window.setInterval(() => main.background(), 1000);

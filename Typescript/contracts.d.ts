interface IStatusContract {
    tunnelConnected: boolean,
    tunnelTrafficHistory: ITrafficMoment[],
    services: IService[]
}


interface ITrafficMoment {
    i: number,
    o: number,
}

interface IService {
    port: number,
    remoteHost: string,
    connections: number,
}
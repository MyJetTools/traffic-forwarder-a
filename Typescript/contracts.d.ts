interface IStatusContract {
    tunnelConnected: boolean,
    tunnelTrafficHistory: ITrafficMoment[]
}


interface ITrafficMoment {
    i: number,
    o: number,
}
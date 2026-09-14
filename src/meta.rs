
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(not(feature = "no-std"), derive(Debug, Hash))]
pub(crate) enum UninhabitedZst {}

/// A type that occupies niches for 255 slots, and then has a value slot for the 256th discriminant.
/// 
/// This can be used to check if a type is an uninhabited ZST.
#[allow(dead_code)]
pub(crate) enum Niche255<T> {
    T(T), Nx01, Nx02, Nx03, Nx04, Nx05, Nx06, Nx07, Nx08, Nx09, Nx0a, Nx0b, Nx0c, Nx0d, Nx0e, Nx0f,
    Nx10, Nx11, Nx12, Nx13, Nx14, Nx15, Nx16, Nx17, Nx18, Nx19, Nx1a, Nx1b, Nx1c, Nx1d, Nx1e, Nx1f,
    Nx20, Nx21, Nx22, Nx23, Nx24, Nx25, Nx26, Nx27, Nx28, Nx29, Nx2a, Nx2b, Nx2c, Nx2d, Nx2e, Nx2f,
    Nx30, Nx31, Nx32, Nx33, Nx34, Nx35, Nx36, Nx37, Nx38, Nx39, Nx3a, Nx3b, Nx3c, Nx3d, Nx3e, Nx3f,
    Nx40, Nx41, Nx42, Nx43, Nx44, Nx45, Nx46, Nx47, Nx48, Nx49, Nx4a, Nx4b, Nx4c, Nx4d, Nx4e, Nx4f,
    Nx50, Nx51, Nx52, Nx53, Nx54, Nx55, Nx56, Nx57, Nx58, Nx59, Nx5a, Nx5b, Nx5c, Nx5d, Nx5e, Nx5f,
    Nx60, Nx61, Nx62, Nx63, Nx64, Nx65, Nx66, Nx67, Nx68, Nx69, Nx6a, Nx6b, Nx6c, Nx6d, Nx6e, Nx6f,
    Nx70, Nx71, Nx72, Nx73, Nx74, Nx75, Nx76, Nx77, Nx78, Nx79, Nx7a, Nx7b, Nx7c, Nx7d, Nx7e, Nx7f,
    Nx80, Nx81, Nx82, Nx83, Nx84, Nx85, Nx86, Nx87, Nx88, Nx89, Nx8a, Nx8b, Nx8c, Nx8d, Nx8e, Nx8f,
    Nx90, Nx91, Nx92, Nx93, Nx94, Nx95, Nx96, Nx97, Nx98, Nx99, Nx9a, Nx9b, Nx9c, Nx9d, Nx9e, Nx9f,
    Nxa0, Nxa1, Nxa2, Nxa3, Nxa4, Nxa5, Nxa6, Nxa7, Nxa8, Nxa9, Nxaa, Nxab, Nxac, Nxad, Nxae, Nxaf,
    Nxb0, Nxb1, Nxb2, Nxb3, Nxb4, Nxb5, Nxb6, Nxb7, Nxb8, Nxb9, Nxba, Nxbb, Nxbc, Nxbd, Nxbe, Nxbf,
    Nxc0, Nxc1, Nxc2, Nxc3, Nxc4, Nxc5, Nxc6, Nxc7, Nxc8, Nxc9, Nxca, Nxcb, Nxcc, Nxcd, Nxce, Nxcf,
    Nxd0, Nxd1, Nxd2, Nxd3, Nxd4, Nxd5, Nxd6, Nxd7, Nxd8, Nxd9, Nxda, Nxdb, Nxdc, Nxdd, Nxde, Nxdf,
    Nxe0, Nxe1, Nxe2, Nxe3, Nxe4, Nxe5, Nxe6, Nxe7, Nxe8, Nxe9, Nxea, Nxeb, Nxec, Nxed, Nxee, Nxef,
    Nxf0, Nxf1, Nxf2, Nxf3, Nxf4, Nxf5, Nxf6, Nxf7, Nxf8, Nxf9, Nxfa, Nxfb, Nxfc, Nxfd, Nxfe, Nxff,
}

extern crate ether_converter;

#[cfg(test)]
mod tests {
    #[test]
    fn convert() {
        let val = "1";
        let unit = "ether";
        let map = ether_converter::convert(&val, &unit);

        let wei = map.get("wei").unwrap();
        assert_eq!(wei.to_string(), "1000000000000000000");

        let kwei = map.get("kwei").unwrap();
        assert_eq!(kwei.to_string(), "1000000000000000");

        let mwei = map.get("mwei").unwrap();
        assert_eq!(mwei.to_string(), "1000000000000");

        let gwei = map.get("gwei").unwrap();
        assert_eq!(gwei.to_string(), "1000000000");

        let szabo = map.get("szabo").unwrap();
        assert_eq!(szabo.to_string(), "1000000");

        let finney = map.get("finney").unwrap();
        assert_eq!(finney.to_string(), "1000");

        let ether = map.get("ether").unwrap();
        assert_eq!(ether.to_string(), "1");

        let kether = map.get("kether").unwrap();
        assert_eq!(kether.to_string(), "0.001");

        let mether = map.get("mether").unwrap();
        assert_eq!(mether.to_string(), "0.000001");

        let gether = map.get("gether").unwrap();
        assert_eq!(gether.to_string(), "0.000000001");

        let tether = map.get("tether").unwrap();
        assert_eq!(tether.to_string(), "0.000000000001");
    }

    #[test]
    fn to_wei() {
        let value = ether_converter::to_wei("1", "ether");
        assert_eq!(value.to_string(), "1000000000000000000");
    }

    #[test]
    fn to_kwei() {
        let value = ether_converter::to_kwei("1", "ether");
        assert_eq!(value.to_string(), "1000000000000000");
    }

    #[test]
    fn to_mwei() {
        let value = ether_converter::to_mwei("1", "ether");
        assert_eq!(value.to_string(), "1000000000000");
    }

    #[test]
    fn to_gwei() {
        let value = ether_converter::to_gwei("1", "ether");
        assert_eq!(value.to_string(), "1000000000");
    }

    #[test]
    fn to_szabo() {
        let value = ether_converter::to_szabo("1", "ether");
        assert_eq!(value.to_string(), "1000000");
    }

    #[test]
    fn to_finney() {
        let value = ether_converter::to_finney("1", "ether");
        assert_eq!(value.to_string(), "1000");
    }

    #[test]
    fn to_ether() {
        let value = ether_converter::to_ether("1", "ether");
        assert_eq!(value.to_string(), "1");
    }

    #[test]
    fn to_kether() {
        let value = ether_converter::to_kether("1", "ether");
        assert_eq!(value.to_string(), "0.001");
    }

    #[test]
    fn to_mether() {
        let value = ether_converter::to_mether("1", "ether");
        assert_eq!(value.to_string(), "0.000001");
    }

    #[test]
    fn to_gether() {
        let value = ether_converter::to_gether("1", "ether");
        assert_eq!(value.to_string(), "0.000000001");
    }

    #[test]
    fn to_tether() {
        let value = ether_converter::to_tether("1", "ether");
        assert_eq!(value.to_string(), "0.000000000001");
    }
}

use crate::{circuit, setup};
use ark_serialize::*;

type E = ark_bls12_381::Bls12_381;
pub fn puzzle_data() -> (circuit::Circuit<E>, setup::Setup<E>) {
    let setup_bytes = b"AwAAAAAAAAC7xiLbCvA6++8aevk/6FVsWKwbFz86TqEFuXSXT\
    4xoww+sqU+MY5UmlNeXMafT8Rdvk4PCl/NeQfg5AKvNWlFkM7FW1lsjJgrRn3R3C+8wq8\
    FktD6mvkmwf3F1+9JwSwryMpvqUDa4qQG0inJxB4xgKpLVLYSe1V5R7PUnVV1GasKBm3H\
    Vps2ARPAPLI4rLYcDAAAAAAAAAPU7Ch6gaKzg53qYOHj5Et0TgHDXbrKuC4rr1/U44FWp\
    BROa2OuU4IqJEKGruRJHFSFYODPyeHFdyzWebVEn8qkxPJuUpntwYSL0om9IT8rWgnQoH\
    YysTaygWmOHfaMYjuzB6Eo91qVYSbwO7ONSGdFl9aHFhFb5eXfmTOKh2mcDZNVj/vHfju\
    WE5bYX119cDAMAAAAAAAAAsaNzD8GUvWFbczVpUlhy2CXiIelqVymMvOOThsCUmJLxBc3\
    my2kRPfDBj4vME0MLX/PLguMHMksFXx5FoZqDf5ZzjnGgkuJFsEbBwjzN1Fk6/QG3MmP4\
    U0fmW8Wt+9MLgXWTkjTxTEF56fRC6ygKqSrql1Yvymq2mjYigwN4g5+AX44GZ26OiFgLz\
    ds6W70FAwAAAAAAAAAhWDgz8nhxXcs1nm1RJ/KpMTyblKZ7cGEi9KJvSE/K1oJ0KB2MrE\
    2soFpjh32jGI7swehKPdalWEm8DuzjUhnRZfWhxYRW+Xl35kziodpnA2TVY/7x347lhOW\
    2F9dfXAz1OwoeoGis4Od6mDh4+RLdE4Bw126yrguK69f1OOBVqQUTmtjrlOCKiRChq7kS\
    RxUDAAAAAAAAAI1llAKMSDr4T1F08uZB8YEMG+XkGp/rhalmnZ7fjPKq+q69pC2KR+ANI\
    8cuYuXWkhN66+ogyBz8I5IBbhPsPO7Hd3w9+fRk21kL9rLy/kgDp+PQMuQy/7c/qhIxcL\
    3mADCGpD4sNC3yY/rOcx82djMjrcgmPpVoHbOD3naSSmDdB1lHUZpHTk+uModPelY1hAM\
    AAAAAAAAANjsFIUF0zJNRW7Y7WIs9xnk6z1rPbheIZyWj4dOyuctpXYd4k0Tvl0BfbOF+\
    6wMNoQc9XZ3wAUmlkQbA2K6O+MB9lTN4e8lKaD+QDMZg13hRC7QrKTG8vLOSgAeFbpoR1\
    wxitRm1REPAtJnSMvcWDwh8sedl1BKHMKC8pNCihEO+E0G7A1NW+yhJUqTzOvuOctcVxy\
    wrVImgIG7vYUHHfX0RzVpswW9ZEgd39jvYQRkrB4mSdAc8d05/77RCVFsXMQp3IUI14UD\
    g42oKBhJG8w2nUlhJZdTT7u9VMOqcpxV5vk60PDqwZD/u7+NV7FEEgdaSlEi7gzuwcYI6\
    O4Xekn8emGe0VFtMurFjhu3ru7hGm4tnFotBfc06A/zOnw0HHLtuEki0r2M9XjhNfCneZ\
    S+dbFu6UNJTAMoqh4dEoApKnQdcl6dqPlq1DVJZHUCATg507HbAm0guc74QOnY09yQOuP\
    jgFZgdykuwGD0CLs40a3Q5/rmgiorbySNTQtYZv6zabfG7MNtJTRRg3iaMgivlrZ/MnPl\
    C6odhpV160p+KoAWl2HLT8mDvA3OtsV6K2CxF2w0GR4Bkx1AEJEJYuRJ5528lKOCBUFxu\
    NuNHLlS5NeXIGseuepH1/2sZC/IUdR5svEsRT1MHZqBbBoW10Mjz/Zye5yAUWE1L3zTC2\
    CkqtSICqmMPG0EEZzLu5biGnoL7ItQMZXCDYjFE/elNMUtnJ206MVpwVQb/eMgwts0g0Y\
    B6vq46/uxBvO3jGbUVJopx5tle7t3B2LFRXNWhvidNypqeLvJSYX31lUcKT7hWoZHrkJk\
    fkKmlSGc7+J6Z";
    (
        circuit::make_circuit::<E>(),
        setup::Setup::<E>::deserialize(&*base64::decode(&setup_bytes).unwrap()).unwrap(),
    )
}

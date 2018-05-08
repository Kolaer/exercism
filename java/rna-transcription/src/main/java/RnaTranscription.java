import java.util.stream.Collectors;

class RnaTranscription {

    String transcribe(String dnaStrand) {
        return dnaStrand.chars()
                .mapToObj(chr -> {
                    switch ((char) chr) {
                        case 'A':
                            return 'U';
                        case 'C':
                            return 'G';
                        case 'T':
                            return 'A';
                        case 'G':
                            return 'C';
                        default:
                            return (char) chr;
                    }
                })
                .map(Object::toString)
                .collect(Collectors.joining());
    }

}

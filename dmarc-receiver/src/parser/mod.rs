pub mod dmarc_definition;
use std::io::{self, BufRead, BufReader};

pub fn parse_stdin() -> Result<Vec<dmarc_definition::feedback>, String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    // skip headers
    let bytes_read = skip_headers(&mut stdin)?;

    stdin.consume(bytes_read);

    let feedback = parse_base64(&mut stdin)?;

    if feedback.len() == 0 {
        return Err(String::from("No DMARC files were found in input"));
    }

    Ok(feedback)
}

fn parse_base64(mut raw_handle: &mut dyn io::Read) -> Result<Vec<dmarc_definition::feedback>, String> {

    // TODO: Does not accept whitespace '\n' within base64 :/ That does not work for us I guess
    // create base64 decoder to decode the stream on-the-fly
    let mut decoded_handle = base64::read::DecoderReader::new(&mut raw_handle, base64::STANDARD);

    let mut dmarc_files = Vec::<dmarc_definition::feedback>::new();

    loop {
        // for each file in zipfile-archive:
        match zip::read::read_zipfile_from_stream(&mut decoded_handle)
            .map_err(|e| format!("Error encountered while reading zip: {:?}", e))?
        {
            Some(contained_file) => {
                dmarc_files.push(
                    serde_xml_rs::from_reader(contained_file)
                        .map_err(|_| "Could not parse dmarc XML file")?
                    );
            },
            None => return Ok(dmarc_files),
        }
    }
}

fn skip_headers(handle: &mut dyn io::Read) -> Result<usize, String> {
    let mut reader = BufReader::new(handle);
    let mut bytes_read = 0;
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)
            .map_err(|_| "Could not read line from email")?;

        bytes_read += len;
        
        if len == 0 {
            return Err(String::from("Could not find message body"));
        }

        if line == "\n" {
            return Ok(bytes_read);
        }       
    }
}

#[cfg(test)]
mod test {

    const HOLYSHIP_TEST_INPUT: &'static str = "
UEsDBAoAAAAIAC9I7VC/t6AO8QEAALAEAAAwAAAAZ29vZ2xlLmNvbSFob2x5c2hpcC5ubCExNTk0
NTEyMDAwITE1OTQ1OTgzOTkueG1srVTBcpswEL3nKzy+GwEG13QUpad+QXtmZLGAJiCpkkjiv+8S
CYyTzPTSE+Lt7tt9Twv06W0cdi9gndTqcZ8l6X4HSuhGqu5x//vXz8N5v3tiD7QFaC5cPLOH3Y5a
MNr6egTPG+75jCGqbVcrPgLrtO4GSIQeKVnBkAMjlwNTGhmG66EZuRUHN5mZ7se2LOTFmjdveS20
8lz4WqpWs957474TEkuTWynhhCv3CpbkxelUnlPk+lwfiKMM2bAyOxZV+q3MsvJYHdNzdqoouYVD
OkqF2nLVRTEIXaCTimVlVZRZnqbYLCBLHFQTotX5WCHl/B7IyD3b2m3rKTV6kOJam+kySNfDOohG
dxTr9XB1vTSJGpAuYCGBN89yZJaScIigM+07Nj8DZNifieMMXiqgxETU3cNuwY3wLJs1zof3mb+a
D10V2i6jWv26muH0ZAXU0iBLkeTFOanypDpihzWwpAo9KWxGSTgscOwHL3yY0L5mCcyeSGe0kx7X
GNdrnnuLbPJmQwx3DhNWb6LsNgZWgzYaP/TE+1qUUdkAWtVK/IjWsh54A7ZurR7v72kbiEyf6imf
fF9bcNPgb5Qfxv3XEsQNnzmirPiyUQwDCK8ty1Nc3iI/ofQFWvVv29KNM/9hhI3TuJYfNM/JYZEo
uf18/gJQSwECCgAKAAAACAAvSO1Qv7egDvEBAACwBAAAMAAAAAAAAAAAAAAAAAAAAAAAZ29vZ2xl
LmNvbSFob2x5c2hpcC5ubCExNTk0NTEyMDAwITE1OTQ1OTgzOTkueG1sUEsFBgAAAAABAAEAXgAA
AD8CAAAAAA==
";

    #[test]
    fn name() {
        let mut handle = std::io::Cursor::new(HOLYSHIP_TEST_INPUT);
        let reports = super::parse_base64(&mut handle).unwrap();

        assert_eq!(1, reports.len());
        assert_eq!(1, reports[0].record.iter().map(|x| x.row.count).sum::<u32>());
        assert_eq!("holyship.nl", reports[0].policy_published.domain);
    }

}
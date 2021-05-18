pub mod dmarc_definition;
use std::io::{self, BufRead, BufReader};

pub fn parse(mut handle: &mut dyn io::BufRead) -> Result<Vec<dmarc_definition::feedback>, String> {

    // skip headers
    skip_headers(&mut handle)?;

    let feedback = parse_base64(&mut handle)?;

    if feedback.len() == 0 {
        return Err(String::from("No DMARC files were found in input"));
    }

    Ok(feedback)
}

fn parse_base64(raw_handle: &mut dyn io::BufRead) -> Result<Vec<dmarc_definition::feedback>, String> {

    let mut dmarc_files = Vec::<dmarc_definition::feedback>::new();

    // b64 encoded zipfile to string
    let mut b64_file = String::new();
    raw_handle.read_to_string(&mut b64_file).map_err(|_| "Could not read body part of email")?;

    // strip whitespace from b64 string
    b64_file.retain(|c| !c.is_whitespace());

    let mut stream = io::Cursor::new(b64_file);

    let mut decoded_stream = base64::read::DecoderReader::new(&mut stream, base64::STANDARD);

    loop {
        // for each file in zipfile-archive:
        match zip::read::read_zipfile_from_stream(&mut decoded_stream)
            .map_err(|e| format!("Error encountered while reading zip: {:?}", e))?
        {
            Some(contained_file) => {
                dmarc_files.push(
                    // deserialize XML report file
                    serde_xml_rs::from_reader(contained_file)
                        .map_err(|_| "Could not parse dmarc XML file")?
                    );
            },
            None => return Ok(dmarc_files),
        }
    }
}

fn skip_headers(handle: &mut dyn io::Read) -> Result<(), String> {
    let mut reader = BufReader::new(handle);
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)
            .map_err(|_| "Could not read line from email")?;
        
        if len == 0 {
            return Err(String::from("Could not find message body"));
        }

        if line == "\n" {
            return Ok(());
        }       
    }
}

#[cfg(test)]
mod test {

    use std::io::Read;

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

    const EMAIL_HEADERS: &'static str = "X-RANDOM-HEADER1: lala
X-RANDOM-HEADER2:
    _value_of_header_2
X-Header-3


";

    #[test]
    fn contents_only() {
        let mut handle = std::io::Cursor::new(HOLYSHIP_TEST_INPUT);
        let reports = super::parse_base64(&mut handle).unwrap();

        assert_eq!(1, reports.len());
        assert_eq!(1, reports[0].record.iter().map(|x| x.row.count).sum::<u32>());
        assert_eq!("holyship.nl", reports[0].policy_published.domain);
    }

    #[test]
    fn entire_mail() {
        let headers = std::io::Cursor::new(EMAIL_HEADERS);
        let file = std::io::Cursor::new(HOLYSHIP_TEST_INPUT);

        let mut handle = headers.chain(file);

        let reports = super::parse(&mut handle).unwrap();

        assert_eq!(1, reports.len());
        assert_eq!(1, reports[0].record.iter().map(|x| x.row.count).sum::<u32>());
        assert_eq!("holyship.nl", reports[0].policy_published.domain);
    }

}
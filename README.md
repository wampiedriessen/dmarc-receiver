# dmarc-receiver
A docker container that receives DMARC reports over SMTP, then exports them to a range of possible destinations.

### Run Docker Container
- Have an MX record pointing to an IP your Docker container can be reached over
- Configure your own `smtpd.conf`, an annotated example can be found at [example-smtpd.conf]
- Run the docker container, approximately something like the following (much dependant on your configuration of `smtpd.conf`)
```
docker run -d \
    --restart always \
    --name dmarc-receiver \
    -p 25:25 \
    -p 587:587 \
    -v my_cert.fullchain.pem:/etc/ssl/dmarc.example.com.fullchain.pem:ro \
    -v my_cert.key:/etc/ssl/private/dmarc.example.com.key:ro \
    wampiedriessen/dmarc-receiver
```

### Configure your SMTPD.conf
TODO: Add links to "general" configuration of smtpd.conf file.
Edit the action called `"dmarc_receive"` the `dmarc-receiver` executable requires the export module and the domain for which the report should be registered

## Export Modules

### Webhook
`"webhook https://example.com/api/ding"` This configures dmarc-receiver to forward dmarc reports to the endpoint provided in the [JSON.md](JSON Format)

### TODO: Add More Exporters
bla bla exporter
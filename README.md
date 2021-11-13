jestas

Specify a configuration file:

    jestas --config myConfig.json

Pass Jenkins server URL manually (if everyone has read access):

    jestas --server https://jenkins.qa.ubuntu.com

Pass server URL, username and API token (for authenticated use):

    jestas --server http://my.jenkins.url --user clarence-oveur --token foo123xyz

Allow Jenkins server with a self-signed certificate:

    jestas --server https://my.jenkins.url --trust

By default, the status of all build jobs is listed.

Giving a job name as a parameter will display the latest (possibly partial) build log:

List jobs with node and win
    jestas node win


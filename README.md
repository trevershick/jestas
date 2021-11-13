jestas

    $ jestas --server http://localhost:8080
    # returns nothing for now

    $ jestas --server http://localhost:8080 jobs
    job1 fail http://localhost:8080/job/job1/

    $ jestas --server http://localhost:8080 jobs --recursive
    job1 fail http://localhost:8080/job/job1/
    BadJob fail http://localhost:8080/job/TestCo/job/BadJob/
    GitProject ok http://localhost:8080/job/TestCo/job/GitProject/
    job3 ok http://localhost:8080/job/TestCo/job/job3/

    $ jestas --server http://localhost:8080 jobs --recursive 3
    job3 ok http://localhost:8080/job/TestCo/job/job3/

Print the Logs
----

'job3' is a job in a folder, so --recursive is necessary

    $ jestas --server http://localhost:8080 logs --recursive job3
    job3 ok http://localhost:8080/job/TestCo/job/job3/
    ------------------------------------------
    Started by user trever shick
    Running as SYSTEM
    Building in workspace /var/jenkins_home/workspace/TestCo/job3
    Finished: SUCCESS


Configuration
----

    cat ~/.jestas.toml

    server = "http://localhost:8080"

    [jobs]
    recursive = true




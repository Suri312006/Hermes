# Hermes
UCSC CSE 108c Project

`./sparta`
- The core SPARTA implementation which runs completely inside an enclave.

`./trojan`
- GRPC proxy to allow clients to connect with sparta.

`./athens`
- CLI and Proxy code to interact with trojan.

`./agora` - agora = meeting place
- the common crate that defines some constants about the entire system

`./sator` - sator = seeding place
- Benchmarking harness for sparta's performance

Note: If you have any trouble with the following instructions, please email me at sjammish@ucsc.edu.

## Steps to running SPARTA

1. Create an AWS Nitro EC2 instance

	```bash
	aws ec2 run-instances \\
  --image-id ami-04acda42f3629e02b \\
  --count 1 \\
  --instance-type m5.xlarge \\
  --key-name <Your key name here> \\
  --enclave-options 'Enabled=true' \\
  ```

 Here are the <a href="https://docs.aws.amazon.com/cli/v1/userguide/cli-services-ec2-instances.html">AWS docs</a>

2. Expand the root partition of your EC2 instance to atleast 32GB. (Probably want to use the console for this)

 Here are the <a href="https://docs.aws.amazon.com/ebs/latest/userguide/recognize-expanded-volume-linux.html">AWS docs</a>

3. Configure Security Groups for your EC2 instance by allowing traffic in and out of port 50051 for all tcp
traffic coming from anywhere, also probably on port 22 to allow for ssh.

 Here are the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-security-groups.html">AWS docs</a>

2. Connect to your EC2 instance via ssh
 Here are the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-connect-methods.html">AWS docs</a>

3. Install the Just command runner

  ```bash
  # create ~/bin
  mkdir -p ~/bin

  # download and extract just to ~/bin/just
  curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin

  # add `~/bin` to the paths that your shell searches for executables
  # this line should be added to your shells initialization file,
  # e.g. `~/.bashrc` or `~/.zshrc`
  export PATH="$PATH:$HOME/bin"

  # just should now be executable
  just --help  
  ```

4. Run `just init`, which will install the necessary packages for you.

5. Then run `just sparta`, which will start both the enclave and the trojan proxy for you.

## Steps for running benchmarks

1. Set the constants you want for the system in `./agora/src/lib.rs`.
2. Run `just kill` to kill the enclave and trojan if you already have them running.
3. Run `just sparta` to compile sparta with the new constants.
4. Run `just bench` to run the benchmark



## Steps to run the entire system
1. Ensure you have sparta running.
2. Set the `TROJAN_IP` variable in `./agora/src/lib.rs` to the public ip address of your
EC2 instance.
3. Ensure you have the right ports open for your instance, as mentioned earlier.
4. Install rust on the computer you want to run the proxy on.
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
5. Install the protobuf compiler for your system.
  - `https://grpc.io/docs/protoc-installation/`
6. Install the just command runner on your local computer.
  - `cargo install just`
7. Run `just proxy create-user` to create your user

8. Run `just cli register` on any device you want to use.

9. Run `just proxy add-device -k <KEY>` where KEY is the output of the previous command.

10. Run `just proxy run -h` to see what arguments the run command takes in.

10. Run `just proxy run ....` with whichever args you like!

11. Run `just cli -h` to see what options you have from the cli!

12. Use however you want now!








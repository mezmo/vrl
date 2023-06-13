library "magic-butler-catalogue"
def PROJECT_NAME = "vrl"
def DEFAULT_BRANCH = "main"
def CURRENT_BRANCH = [env.CHANGE_BRANCH, env.BRANCH_NAME]?.find{branch -> branch != null}
def DRY_RUN = CURRENT_BRANCH != DEFAULT_BRANCH

pipeline {
    agent {
        node {
            label "ec2-fleet"
            customWorkspace("/tmp/workspace/${env.BUILD_TAG}")
        }
    }

    parameters {
        string(name: "SANITY_BUILD", defaultValue: "", description: "Is this a scheduled sanity build that skips releasing?")
    }

    triggers {
        parameterizedCron(
            // Cron hours are in GMT, so this is roughly 12-3am EST, depending on DST
            env.BRANCH_NAME == DEFAULT_BRANCH ? "H H(5-6) * * * % SANITY_BUILD=true" : ""
        )
    }

    options {
        timeout time: 1, unit: "HOURS"
        timestamps()
        ansiColor "xterm"
    }

    environment {
        GITHUB_TOKEN = credentials("github-api-token")
        NPM_CONFIG_CACHE = ".npm"
        NPM_CONFIG_USERCONFIG = ".npm/rc"
        SPAWN_WRAP_SHIM_ROOT = ".npm"
        RUSTUP_HOME = "/opt/rust/cargo"
        CARGO_HOME = "/opt/rust/cargo"
        PATH = """${sh(
           returnStdout: true,
           script: 'echo /opt/rust/cargo/bin:\$PATH'
        )}
        """
        // for the semantic-release-rust executable, we must have this set even when not publishing the crate directly
        CARGO_REGISTRY_TOKEN = "not-in-use"
    }

    post {
        always {
            script {
                jiraSendBuildInfo site: "logdna.atlassian.net"

                if (env.SANITY_BUILD == "true") {
                    notifySlack(
                        currentBuild.currentResult,
                        [
                            channel: "#pipeline-bots",
                            tokenCredentialId: "qa-slack-token"
                        ],
                        "`${PROJECT_NAME}` sanity build took ${currentBuild.durationString.replaceFirst(' and counting', '')}."
                    )
                }
            }
        }
    }

    stages {
        stage("Validate") {
            tools {
                nodejs "NodeJS 18"
            }

            steps {
                script {
                    sh "mkdir -p ${NPM_CONFIG_CACHE}"
                    npm.auth token: GITHUB_TOKEN
                    sh "npx @answerbook/commitlint-config-logdna"
                }
            }
        }

        stage("Test") {
            when {
                beforeAgent true
                not {
                    changelog "\\[skip ci\\]"
                }
            }

            parallel {
                stage("Rust Unit Tests") {
                    steps {
                        script {
                            sh "docker build --progress=plain --target base ."
                        }
                    }
                }

                stage("Wasm Unit Tests") {
                    steps {
                        script {
                            sh "docker build --progress=plain --target wasm-base ."
                        }
                    }
                }

                stage("Release Test") {
                    when {
                        beforeAgent true
                        not {
                            branch DEFAULT_BRANCH
                        }
                    }

                    environment {
                        GIT_BRANCH = "${CURRENT_BRANCH}"
                        BRANCH_NAME = "${CURRENT_BRANCH}"
                        CHANGE_ID = ""
                    }

                    tools {
                        nodejs 'NodeJS 18'
                    }

                    steps {
                        script {
                            sh "mkdir -p ${NPM_CONFIG_CACHE}"
                            npm.auth token: GITHUB_TOKEN
                            sh "cargo install semantic-release-cargo --version 2.1.92"
                            sh "npm install -G semantic-release@^19.0.0 @semantic-release/git@10.0.1 @semantic-release/changelog@6.0.3 @semantic-release/exec@6.0.3 @answerbook/release-config-logdna@2.0.0"
                            sh 'npx semantic-release --dry-run --no-ci --branches=${BRANCH_NAME:-main}'
                        }
                    }
                }
            }
        }

        stage("Release") {
            when {
                beforeAgent true
                branch DEFAULT_BRANCH
                not {
                    allOf {
                        changelog "\\[skip ci\\]"
                        environment name: "SANITY_BUILD", value: "true"
                    }
                }
            }

            tools {
                nodejs "NodeJS 18"
            }

            steps {
                script {
                    sh "mkdir -p ${NPM_CONFIG_CACHE}"
                    npm.auth token: GITHUB_TOKEN
                    sh "cargo install semantic-release-cargo --version 2.1.92"
                    sh "npm install -G semantic-release@^19.0.0 @semantic-release/git@10.0.1 @semantic-release/changelog@6.0.3 @semantic-release/exec@6.0.3 @answerbook/release-config-logdna@2.0.0"
                    sh "npx semantic-release"

                    def RELEASE_VERSION = sh(
                        returnStdout: true,
                        script: "cargo metadata -q --no-deps --format-version 1 | jq -r \'.packages[0].version\'"
                    ).trim()

                    // Only the wasm module needs to be published. The rest of this repo is used as a cargo dependency
                    // and that does not require publishing the library anywhere. Cargo just uses git urls to find the
                    // dependency.
                    sh "docker build --progress=plain --build-arg GITHUB_TOKEN=${GITHUB_TOKEN} --build-arg DRY_RUN=${DRY_RUN} --target wasm-publish ."
                }
            }
        }
    }
}

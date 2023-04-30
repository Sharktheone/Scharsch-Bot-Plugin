plugins {
    java
    kotlin("jvm") version "1.7.20"
    id("com.github.johnrengelman.shadow") version "8.1.1"
}
configurations {
    create("kotlin")
}


tasks.withType<JavaCompile>() {
    options.encoding = "UTF-8"
}

tasks.withType<Javadoc>() {
    options.encoding = "UTF-8"
}

repositories {
    mavenLocal()
    maven {
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }

    maven {
        url = uri("https://oss.sonatype.org/content/groups/public/")
    }

    maven {
        url = uri("https://repo.maven.apache.org/maven2/")
    }

    dependencies {
        api("org.jetbrains.kotlin:kotlin-stdlib-jdk8:1.7.20")
        testImplementation("org.jetbrains.kotlin:kotlin-test:1.7.20")
        compileOnly("io.papermc.paper:paper-api:1.19-R0.1-SNAPSHOT")
    }

    group = "de.scharschbot"
    version = "1.0-SNAPSHOT"
    description = "ScharschBotPlugin"
    java.sourceCompatibility = JavaVersion.VERSION_1_8

    sourceSets {
        main {
            kotlin {
                srcDirs("src/main/kotlin")
            }
            resources {
                srcDirs("src/main/jniLibs")
            }
        }
    }

    tasks {
        compileJava {
            options.encoding = "UTF-8"
        }
        compileKotlin {
            kotlinOptions.jvmTarget = "1.8"
        }
        build {
            dependsOn("shadowJar")
        }

        shadowJar {
            manifest {
                attributes["Main-Class"] = "de.scharschbot.plugin.Plugin"
            }
            relocate("kotlin", "de.scharschbot.plugin.kotlin")
        }
    }


    kotlin {
        jvmToolchain(17)
    }

    tasks.register("generateTemplates") {
    }
}
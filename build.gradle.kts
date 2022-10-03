plugins {
    kotlin("multiplatform") version "1.7.20"
    application
}

group = "sh.adamcooper"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

kotlin {
    jvm("java") {
        compilations.all {
            kotlinOptions.jvmTarget = "16"
        }
        withJava()
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
    }
    sourceSets {
        val javaMain by getting {
            kotlin.srcDir("src/main/kotlin")
            resources.srcDir("src/main/resources")
        }
    }
}

application {
    mainClass.set("sh.adamcooper.wordle.MainKt")
}

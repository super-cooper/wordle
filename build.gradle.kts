import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    alias(libs.plugins.kotlin.multiplatform)
}

group = "sh.adamcooper"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

kotlin {
    jvm("java") {
        compilerOptions {
            jvmTarget = JvmTarget.JVM_23
        }
    }
    sourceSets {
        val javaMain by getting {
            kotlin.srcDir("src/main/kotlin")
            resources.srcDir("src/main/resources")
        }
    }
}

tasks.register<JavaExec>("run") {
    val javaTarget = kotlin.targets.getByName("java")
    val mainCompilation = javaTarget.compilations.getByName("main")

    classpath = mainCompilation.output.allOutputs + configurations.getByName("javaRuntimeClasspath")
    mainClass.set("sh.adamcooper.wordle.MainKt")

    // Ensure the code is compiled before running
    dependsOn(mainCompilation.compileTaskProvider)
}

import org.jetbrains.kotlin.gradle.ExperimentalKotlinGradlePluginApi
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import com.vanniktech.maven.publish.SonatypeHost

val libraryName = "Scodok"
version = "0.1.0"

plugins {
  id("org.jetbrains.kotlin.multiplatform") version "2.0.0"
  id("com.vanniktech.maven.publish") version "0.29.0"
  id("com.android.library") version "8.3.0"
}

kotlin {
  jvm()
  androidTarget {
    publishLibraryVariants("release")
    @OptIn(ExperimentalKotlinGradlePluginApi::class)
    compilerOptions { jvmTarget.set(JvmTarget.JVM_1_8) }
  }

  sourceSets {
    val commonMain by getting {
      dependencies {
        // https://mozilla.github.io/uniffi-rs/0.28/kotlin/gradle.html#coroutines-dependency
        implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
      }
    }
    val androidMain by getting {
      dependencies {
        implementation("net.java.dev.jna:jna:5.16.0@aar")
      }
    }
    val jvmMain by getting {
      dependencies {
        implementation("net.java.dev.jna:jna:5.16.0")
      }
    }
  }
}

val groupName = "ink.literate"
val idLibraryName = libraryName.lowercase()

group = groupName

android {
  namespace = groupName
  compileSdk = 34
  defaultConfig { minSdk = 24 }
}

mavenPublishing {
  coordinates(groupName, idLibraryName, version.toString())

  pom {
    name = libraryName
    description = "A simple API wrapper for ScoDoc Notes."
    inceptionYear = "2024"

    url = "https://docs.literate.ink/$idLibraryName"

    licenses {
      license {
        name.set("GPL-3.0-or-later")
        url.set("https://www.gnu.org/licenses/gpl-3.0.txt")
        distribution.set("https://www.gnu.org/licenses/gpl-3.0.txt")
      }
    }

    developers {
      developer {
        organization = "LiterateInk"
        organizationUrl = "https://literate.ink"
      }
    }

    scm {
      url = "https://github.com/LiterateInk/$libraryName"
      connection = "scm:git:https://github.com/LiterateInk/$libraryName.git"
      developerConnection = "scm:git:https://github.com/LiterateInk/$libraryName.git"
    }
  }

  publishToMavenCentral(SonatypeHost.CENTRAL_PORTAL, automaticRelease = true)
  signAllPublications()
}

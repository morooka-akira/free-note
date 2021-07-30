import dependencies.Deps
plugins {
    id("com.android.application")
    id("kotlin-android")
    kotlin("kapt")
}
//apply(mapOf("plugin" to "kotlin-kapt"))

android {
    compileSdk = 30

    defaultConfig {
        applicationId = "com.inon29.example.roomwordsample"
        minSdk = 26
        targetSdk = 30
        versionCode = 1
        versionName = "1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    packagingOptions {
        resources {
            excludes.add("META-INF/atomicfu.kotlin_module")
        }
    }
}

dependencies {
    implementation(Deps.appcompat)
    implementation(Deps.coreKtx)
    implementation(Deps.activityKtx)

    // Room components
    implementation(Deps.roomKtx)
    implementation("androidx.appcompat:appcompat:1.3.1")
    implementation("com.google.android.material:material:1.4.0")
    implementation("androidx.constraintlayout:constraintlayout:2.0.4")
    kapt(Deps.roomCompiler)
    androidTestImplementation(Deps.roomTesting)

    // Lifecycle components
    implementation(Deps.viewModelKtx)
    implementation(Deps.livedataKtx)
    implementation(Deps.commonJava8)

    // Kotlin components
    implementation(Deps.kotlinStdlibJdk)
    api(Deps.coroutinesCore)
    api(Deps.coroutinesAndroid)

    // UI
    implementation(Deps.material)
    implementation(Deps.constraintLayout)

    // Testing
    testImplementation(Deps.junit)
    androidTestImplementation(Deps.coreTesting)
    androidTestImplementation(Deps.espresso) {
        exclude(
            group = "com.android.support",
            module = "support-annotations"
        )
    }
    androidTestImplementation(Deps.androidxJunit)
}
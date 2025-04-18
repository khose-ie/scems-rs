#                                
#     ___  ___ ___ _ __ ___  ___ 
#    / __|/ __/ _ \ '_ ` _ \/ __|
#    \__ \ (_|  __/ | | | | \__ \
#    |___/\___\___|_| |_| |_|___/
#                                
#  

PROFILE ?= debug

BASE ?= $(CURDIR)
ARCH ?= thumbv7em-none-eabihf

# Check the OS.
# If windows, use PowerShell command.
ifeq ($(OS),Windows_NT)
	ECHO     = echo
	RM       = powershell -Command "Remove-Item -Recurse -Force -ErrorAction SilentlyContinue"
	CONTINUE = || true
# By default, the Linux, use the shell.
else
	ECHO     = echo
	RM       = rm -rf
	CONTINUE = || true
endif

# Set the common used toolkit path.
CARGO = cargo
CMAKE = cmake

# RUST app build parameter.
APP      ?= main
APP_PATH ?= .

RUST_BUILD      ?= target
RUST_BUILD_TYPE ?= 
RUST_LIB_PATH   ?= $(BASE)/$(APP_PATH)/$(RUST_BUILD)/$(ARCH)/$(PROFILE)/lib$(APP).a

# RUST Cargo build profile parameter.
ifeq ($(PROFILE),release)
  RUST_BUILD_TYPE = --release
endif

# Set the platform settings.
PLATFORM      ?= challen-v2-f429
PLATFORM_PATH ?= platform/${PLATFORM}

# CMake platform build parameters.
CMAKE_BUILD      ?= build
CMAKE_BUILD_TYPE ?= Debug

ifeq ($(PROFILE),release)
  CMAKE_BUILD_TYPE = Release
endif

APP_INIT_FILE ?= $(BASE)/$(PLATFORM_PATH)/$(PLATFORM).initialize.c

all: platform_with_app

# Build the RUST based app code and generate a static library.
app:
	@$(ECHO) "-----------------------------   Compile App   -----------------------------"
	@$(CARGO) build --manifest-path $(APP_PATH)/Cargo.toml ${RUST_BUILD_TYPE} --features ${PLATFORM} --target ${ARCH}

# Clean the app generated target.
clean-app:
	@$(CARGO) clean --manifest-path $(APP_PATH)/Cargo.toml

# Build the C based platform code and generate the embedded image.
platform:
	@$(ECHO) "----------------------------- Compile Platform -----------------------------"
	$(CMAKE) -S $(PLATFORM_PATH) -B $(CMAKE_BUILD) --preset $(CMAKE_BUILD_TYPE) -DAPP_INIT_FILE=$(APP_INIT_FILE)
	@$(CMAKE) --build $(CMAKE_BUILD)

# Build the C based platform with RUST based app code and generate the embedded image.
platform_with_app: app
	@$(ECHO) ""

	@$(ECHO) "----------------------------- Compile Platform -----------------------------"
	@$(CMAKE) -S $(PLATFORM_PATH) -B $(CMAKE_BUILD) --preset $(CMAKE_BUILD_TYPE) -DAPP_INIT_FILE=$(APP_INIT_FILE) -DAPP_MAIN_LIB=$(RUST_LIB_PATH)
	@$(CMAKE) --build $(CMAKE_BUILD)

# Clena the platform generated target.
clean-platform:
	@$(MAKE) -C $(CMAKE_BUILD) clean $(CONTINUE)

# Clean the built target
clean: clean-platform clean-app

# Dist clean to direct remove target folder
distclean:
	@$(RM) $(CMAKE_BUILD) $(CONTINUE)
	@$(RM) $(RUST_BUILD) $(CONTINUE)

# Build image with the release profile.
release:
	@$(MAKE) PROFILE=release $(MAKEOVERRIDES)

.PHONY: all app clean release platform

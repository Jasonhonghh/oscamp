# Changelog

本项目fork自[os_camp](https://github.com/arceos-org/oscamp) 2024-11-9

所有的修改都会记录在这个文件中。changelog格式参考[Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
## [0.0.2] - 2024-11-14
### Added
- 新建exercise3分支，用于实验3。

## [0.0.2] - 2024-11-10
### Added
- 新建exercise分支，用于实验1、2，此main分支用于同步主项目。

## [0.0.1] - 2024-11-9
### Added
- arceos/cargo.toml中添加members=["examples/httpserver"], 用于编译examples/httpserver。
### Changed
- 修改arceos/scripts/utils.mk中的mk_pflash函数的count(32->64), 使得可以支持更大的pflash。


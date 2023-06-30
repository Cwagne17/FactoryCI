package main

import (
	"factory_agent/pkg"
	"io/ioutil"

	"github.com/hashicorp/hcl/v2"
	"github.com/hashicorp/hcl/v2/hclsyntax"
)

type Config struct {
	LogLevel string `hcl:"log_level"`
}

func main() {
	var inputPath = "../examples/.factory/full-syntax-example.hcl"
	content, err := ioutil.ReadFile(inputPath)
	if err != nil {
		panic(err)
	}
	file, diags := hclsyntax.ParseConfig(content, inputPath, hcl.Pos{Line: 1, Column: 1, Byte: 0})
	if diags != nil && diags.HasErrors() {
		println(diags.Errs()[0].Error())
	}

	out, decodeErr := pkg.Decode(file.Body)
	if decodeErr != nil {
		println(decodeErr.Error())
	}
	out.Execute()
}

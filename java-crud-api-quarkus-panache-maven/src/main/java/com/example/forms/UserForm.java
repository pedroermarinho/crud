package com.example.forms;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotEmpty;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

public record UserForm(

        @Schema(required = true, description = "nome do usuário")
        @NotEmpty
        String name,
        @Schema(required = true, description = "email do usuário")
        @NotEmpty
        @Email
        String email
) {

}

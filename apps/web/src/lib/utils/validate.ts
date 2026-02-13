type ValidationRule<T> = {
	check: (value: T) => boolean;
	message: string;
};

type FieldRules<T> = {
	[K in keyof T]?: ValidationRule<T[K]>[];
};

export type ValidationErrors<T> = Partial<Record<keyof T, string>>;

export function validate<T extends Record<string, unknown>>(
	data: T,
	rules: FieldRules<T>
): { valid: boolean; errors: ValidationErrors<T> } {
	const errors: ValidationErrors<T> = {};

	for (const [field, fieldRules] of Object.entries(rules)) {
		const value = data[field];
		for (const rule of fieldRules as ValidationRule<unknown>[]) {
			if (!rule.check(value)) {
				(errors as Record<string, string>)[field] = rule.message;
				break;
			}
		}
	}

	return { valid: Object.keys(errors).length === 0, errors };
}

// ── Common Rules ──

export const required = (msg = 'This field is required'): ValidationRule<unknown> => ({
	check: (v) => v !== null && v !== undefined && v !== '',
	message: msg
});

export const minLength = (min: number, msg?: string): ValidationRule<string> => ({
	check: (v) => typeof v === 'string' && v.length >= min,
	message: msg ?? `Must be at least ${min} characters`
});

export const maxLength = (max: number, msg?: string): ValidationRule<string> => ({
	check: (v) => typeof v === 'string' && v.length <= max,
	message: msg ?? `Must be at most ${max} characters`
});

export const email = (msg = 'Invalid email address'): ValidationRule<string> => ({
	check: (v) => /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v ?? ''),
	message: msg
});

export const phone = (msg = 'Invalid phone number'): ValidationRule<string> => ({
	check: (v) => /^[\d\s\-\(\)\+\.]{7,20}$/.test(v ?? ''),
	message: msg
});

export const numeric = (msg = 'Must be a number'): ValidationRule<string> => ({
	check: (v) => !isNaN(Number(v)),
	message: msg
});

export const min = (minVal: number, msg?: string): ValidationRule<number | string> => ({
	check: (v) => Number(v) >= minVal,
	message: msg ?? `Must be at least ${minVal}`
});

export const max = (maxVal: number, msg?: string): ValidationRule<number | string> => ({
	check: (v) => Number(v) <= maxVal,
	message: msg ?? `Must be at most ${maxVal}`
});

export const pattern = (regex: RegExp, msg = 'Invalid format'): ValidationRule<string> => ({
	check: (v) => regex.test(v ?? ''),
	message: msg
});

export const optional = <T>(rule: ValidationRule<T>): ValidationRule<T | null | undefined | ''> => ({
	check: (v) => v === null || v === undefined || v === '' || rule.check(v as T),
	message: rule.message
});

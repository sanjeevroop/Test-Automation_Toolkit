import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/30032018/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Temp/30032018/input_Password'), '1234')

WebUI.click(findTestObject('Temp/30032018/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Temp/30032018/h4_YOUR SETUP'))

WebUI.click(findTestObject('Temp/30032018/button_Your Setup'))

WebUI.click(findTestObject('Temp/30032018/a_Your Payroll'))

WebUI.click(findTestObject('Temp/30032018/a_Payroll Basic Details'))

WebUI.selectOptionByValue(findTestObject('Screens/Payroll Companies/select_testDev Payroll 1WebTes'), '30e66e53-4f3c-4a03-9e1b-191b06a117a3', 
    true)

WebUI.selectOptionByIndex(findTestObject('Temp/30032018/select_Please selectAnnualFort'), findTestData('Basic Details').getValue(
        1, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.Address1'), findTestData('Basic Details').getValue(2, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.Address2'), findTestData('Basic Details').getValue(3, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.Address3'), findTestData('Basic Details').getValue(4, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.City'), findTestData('Basic Details').getValue(5, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.County'), findTestData('Basic Details').getValue(6, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.Country'), findTestData('Basic Details').getValue(7, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.PostCode'), findTestData('Basic Details').getValue(8, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.PhoneNo'), findTestData('Basic Details').getValue(9, 1))

WebUI.setText(findTestObject('Temp/30032018/input_PayrollCompany.Statutory'), findTestData('Basic Details').getValue(10, 1))

WebUI.selectOptionByIndex(findTestObject('Temp/30032018/select_UKJerseyGuernseyGibralt'), findTestData('Basic Details').getValue(
        11, 1))

WebUI.click(findTestObject('Temp/30032018/button_Next'))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.BankAccou (1)'), findTestData('Basic Details').getValue(12, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.SortCodeO (1)'), findTestData('Basic Details').getValue(13, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.SortCodeT (1)'), findTestData('Basic Details').getValue(14, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.SortCodeT_1 (1)'), findTestData('Basic Details').getValue(15, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.BankAccou_1 (1)'), findTestData('Basic Details').getValue(16, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.BACSUserN (1)'), findTestData('Basic Details').getValue(17, 1))

WebUI.setText(findTestObject('Temp/input_PayrollCompany.BACSLimit (1)'), findTestData('Basic Details').getValue(18, 1))

WebUI.click(findTestObject('Temp/button_Next_1'))


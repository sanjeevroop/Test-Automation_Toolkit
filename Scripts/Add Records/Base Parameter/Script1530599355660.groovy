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

WebUI.setText(findTestObject('Screens/Base Parameters/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/Base Parameters/input_Password'), '1234')

WebUI.click(findTestObject('Screens/Base Parameters/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/Base Parameters/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Base Parameters/button_Your Setup'))

WebUI.click(findTestObject('Screens/Base Parameters/a_Your Payroll'))

WebUI.click(findTestObject('Screens/Base Parameters/a_Payroll Basic Details'))

WebUI.selectOptionByValue(findTestObject('Screens/Base Parameters/select_testDev Payroll 1WebTes'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

WebUI.click(findTestObject('Screens/Base Parameters/button_Next'))

WebUI.click(findTestObject('Screens/Base Parameters/button_Next_1'))

//WebUI.click(findTestObject('Screens/Base Parameters/div_col-md-4 inputGroupContain'))
WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.EmployeeN'), findTestData('Base Parameter').getValue(
        1, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.EmployeeN_1'), findTestData('Base Parameter').getValue(
        2, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectAnnualPeri'), findTestData('Base Parameter').getValue(
        3, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.BasicSala'), findTestData('Base Parameter').getValue(
        4, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.BasicHour'), findTestData('Base Parameter').getValue(
        5, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.FlatRate'), findTestData('Base Parameter').getValue(6, 
        1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.TimeHalf'), findTestData('Base Parameter').getValue(7, 
        1))

//WebUI.click(findTestObject('Screens/Base Parameters/div_Enter value between W010 -'))
WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.DoubleTim'), findTestData('Base Parameter').getValue(
        8, 1))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.MinimumRa'))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.MinimumRa'))
WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectEffective'), findTestData('Base Parameter').getValue(
        10, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Hours  DecimalsHours  M'), findTestData('Base Parameter').getValue(
        11, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_42'), findTestData('Base Parameter').getValue(12, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectAnnual (Mo'), findTestData('Base Parameter').getValue(
        13, 1))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.ABHR (1)'))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.ABHR'))
WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Do not take - no Arrear'), findTestData('Base Parameter').getValue(
        15, 1))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.NegativeN'))
WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.NegativeN (1)'))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_PercentageTable'), findTestData('Base Parameter').getValue(
        17, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectAnnual Cal'), findTestData('Base Parameter').getValue(
        18, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.DailyRate'), findTestData('Base Parameter').getValue(
        19, 1))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio'))
WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio (2)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio_1 (1)'))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio_1'))
WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_UpDown'), findTestData('Base Parameter').getValue(22, 1))

//WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.GapReport'))
WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.GapReport (2)'))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectCR001CR002'), findTestData('Base Parameter').getValue(
        24, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectCR001CR002_1'), findTestData('Base Parameter').getValue(
        25, 1))

WebUI.selectOptionByIndex(findTestObject('Screens/Base Parameters/select_Please selectH01H02H03H'), findTestData('Base Parameter').getValue(
        26, 1))

//WebUI.click(findTestObject('Screens/Base Parameters/div_col-md-4 inputGroupContain'))
WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.Apprentic'), findTestData('Base Parameter').getValue(
        27, 1))

WebUI.setText(findTestObject('Screens/Base Parameters/input_PayrollCompany.EmployerA'), findTestData('Base Parameter').getValue(
        28, 1))

WebUI.click(findTestObject('Screens/Base Parameters/div_input-group'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.SmallEmpl (1)'))

WebUI.click(findTestObject('Screens/Base Parameters/button_Next_2'))

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Testing/input_Username (5)'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Testing/input_Password (5)'), '1234')

WebUI.click(findTestObject('Testing/input_Submit (5)'))

WebUI.click(findTestObject('Testing/a_Your Payroll (5)'))

WebUI.click(findTestObject('Testing/a_Payroll Basic Details (4)'))

WebUI.click(findTestObject('Testing/button_Next (4)'))

WebUI.click(findTestObject('Testing/button_Next_1 (4)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.NegativeN (1)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio (1)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio_1 (1)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.GapReport (1)'))

WebUI.closeBrowser()

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Testing/input_Username (7)'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Testing/input_Password (7)'), '1234')

WebUI.click(findTestObject('Testing/input_Submit (7)'))

WebUI.click(findTestObject('Testing/button_Your Setup (5)'))

WebUI.click(findTestObject('Testing/a_Your Payroll (6)'))

WebUI.click(findTestObject('Testing/a_Payroll Basic Details (6)'))

WebUI.click(findTestObject('Testing/button_Next (6)'))

WebUI.click(findTestObject('Testing/button_Next_1 (6)'))

WebUI.click(findTestObject('Screens/Base Parameters/input_PayrollCompany.Proportio (2)'))

WebUI.closeBrowser()

